/*!
 * Multi-threaded TCP Server
 * 
 * A production-quality concurrent TCP server that handles multiple clients using a thread pool.
 * Features connection management, graceful shutdown, and request/response handling.
 * 
 * # Compile and Run
 * ```bash
 * rustc multi_threaded_server.rs -o multi_threaded_server
 * ./multi_threaded_server
 * ```
 * 
 * # Test with:
 * ```bash
 * # Terminal 1
 * ./multi_threaded_server
 * 
 * # Terminal 2+
 * telnet localhost 7878
 * # Or: echo "Hello" | nc localhost 7878
 * ```
 */

use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

/// Worker represents a thread in the thread pool
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new worker that listens for jobs
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            
            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// ThreadPool manages a pool of worker threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// # Arguments
    /// * `size` - Number of threads in the pool
    /// 
    /// # Panics
    /// Panics if size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a job on the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// Connection statistics
#[derive(Clone)]
struct ServerStats {
    total_connections: Arc<Mutex<u64>>,
    active_connections: Arc<Mutex<u64>>,
}

impl ServerStats {
    fn new() -> Self {
        ServerStats {
            total_connections: Arc::new(Mutex::new(0)),
            active_connections: Arc::new(Mutex::new(0)),
        }
    }

    fn increment_total(&self) {
        *self.total_connections.lock().unwrap() += 1;
    }

    fn increment_active(&self) {
        *self.active_connections.lock().unwrap() += 1;
    }

    fn decrement_active(&self) {
        *self.active_connections.lock().unwrap() -= 1;
    }

    fn get_stats(&self) -> (u64, u64) {
        let total = *self.total_connections.lock().unwrap();
        let active = *self.active_connections.lock().unwrap();
        (total, active)
    }
}

/// Handle a single client connection
fn handle_client(mut stream: TcpStream, stats: ServerStats, addr: SocketAddr) {
    stats.increment_active();
    println!("New connection from: {}", addr);

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    
    // Send welcome message
    if let Err(e) = stream.write_all(b"Welcome to Rust Multi-threaded Server!\n") {
        eprintln!("Failed to send welcome message: {}", e);
        stats.decrement_active();
        return;
    }

    if let Err(e) = stream.write_all(b"Commands: ECHO <msg>, STATS, SLEEP <seconds>, QUIT\n\n") {
        eprintln!("Failed to send help message: {}", e);
        stats.decrement_active();
        return;
    }

    loop {
        let mut line = String::new();
        
        match reader.read_line(&mut line) {
            Ok(0) => {
                // Connection closed
                println!("Client {} disconnected", addr);
                break;
            }
            Ok(_) => {
                let line = line.trim();
                
                if line.is_empty() {
                    continue;
                }

                println!("Received from {}: {}", addr, line);

                let response = process_command(line, &stats);
                
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }

                if line.to_uppercase().starts_with("QUIT") {
                    println!("Client {} requested disconnect", addr);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", addr, e);
                break;
            }
        }
    }

    stats.decrement_active();
}

/// Process client commands
fn process_command(cmd: &str, stats: &ServerStats) -> String {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    
    if parts.is_empty() {
        return "ERROR: Empty command\n".to_string();
    }

    match parts[0].to_uppercase().as_str() {
        "ECHO" => {
            let msg = parts[1..].join(" ");
            format!("ECHO: {}\n", msg)
        }
        "STATS" => {
            let (total, active) = stats.get_stats();
            format!("Total connections: {}, Active connections: {}\n", total, active)
        }
        "SLEEP" => {
            if parts.len() < 2 {
                return "ERROR: SLEEP requires duration in seconds\n".to_string();
            }
            
            match parts[1].parse::<u64>() {
                Ok(secs) => {
                    if secs > 10 {
                        return "ERROR: Maximum sleep time is 10 seconds\n".to_string();
                    }
                    thread::sleep(Duration::from_secs(secs));
                    format!("Slept for {} seconds\n", secs)
                }
                Err(_) => "ERROR: Invalid duration\n".to_string(),
            }
        }
        "QUIT" => {
            "Goodbye!\n".to_string()
        }
        "HELP" => {
            "Commands: ECHO <msg>, STATS, SLEEP <seconds>, QUIT, HELP\n".to_string()
        }
        _ => {
            format!("ERROR: Unknown command '{}'. Type HELP for commands.\n", parts[0])
        }
    }
}

/// Main server
fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr)?;
    let pool = ThreadPool::new(4);
    let stats = ServerStats::new();

    println!("Server listening on {}", addr);
    println!("Press Ctrl+C to shutdown");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                stats.increment_total();
                let stats_clone = stats.clone();
                
                pool.execute(move || {
                    handle_client(stream, stats_clone, addr);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    println!("Server shutting down");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_thread_pool_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_stats() {
        let stats = ServerStats::new();
        stats.increment_total();
        stats.increment_active();
        let (total, active) = stats.get_stats();
        assert_eq!(total, 1);
        assert_eq!(active, 1);
        stats.decrement_active();
        let (_, active) = stats.get_stats();
        assert_eq!(active, 0);
    }

    #[test]
    fn test_command_processing() {
        let stats = ServerStats::new();
        
        let response = process_command("ECHO Hello World", &stats);
        assert!(response.contains("Hello World"));
        
        let response = process_command("STATS", &stats);
        assert!(response.contains("Total connections"));
        
        let response = process_command("UNKNOWN", &stats);
        assert!(response.contains("ERROR"));
    }
}
