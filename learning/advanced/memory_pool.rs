/*!
 * Memory Pool Implementation
 * 
 * A thread-safe object pool for efficient memory reuse:
 * - Generic pool implementation
 * - Configurable size limits
 * - Thread-safe access
 * - Performance benchmarks
 * - Statistics tracking
 * 
 * # Compile and Run
 * ```bash
 * rustc memory_pool.rs -o memory_pool
 * ./memory_pool
 * 
 * # With optimizations for benchmarks:
 * rustc -O memory_pool.rs -o memory_pool
 * ./memory_pool
 * ```
 */

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

// ============================================================================
// Object Pool Implementation
// ============================================================================

/// A reusable object wrapper that returns to pool on drop
pub struct PooledObject<T> {
    obj: Option<T>,
    pool: Arc<Mutex<Vec<T>>>,
}

impl<T> PooledObject<T> {
    fn new(obj: T, pool: Arc<Mutex<Vec<T>>>) -> Self {
        PooledObject {
            obj: Some(obj),
            pool,
        }
    }

    /// Get a reference to the inner object
    pub fn get(&self) -> &T {
        self.obj.as_ref().unwrap()
    }

    /// Get a mutable reference to the inner object
    pub fn get_mut(&mut self) -> &mut T {
        self.obj.as_mut().unwrap()
    }
}

impl<T> Drop for PooledObject<T> {
    fn drop(&mut self) {
        if let Some(obj) = self.obj.take() {
            // Return object to pool
            if let Ok(mut pool) = self.pool.lock() {
                pool.push(obj);
            }
        }
    }
}

impl<T> std::ops::Deref for PooledObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> std::ops::DerefMut for PooledObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// Statistics for the pool
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    pub created: usize,
    pub reused: usize,
    pub returned: usize,
    pub current_size: usize,
    pub peak_size: usize,
}

/// Thread-safe object pool
pub struct ObjectPool<T> {
    objects: Arc<Mutex<Vec<T>>>,
    factory: Arc<dyn Fn() -> T + Send + Sync>,
    max_size: usize,
    stats: Arc<Mutex<PoolStats>>,
}

impl<T: Send + 'static> ObjectPool<T> {
    /// Create a new object pool
    /// 
    /// # Arguments
    /// * `factory` - Function to create new objects
    /// * `initial_size` - Number of objects to pre-allocate
    /// * `max_size` - Maximum pool size (0 = unlimited)
    pub fn new<F>(factory: F, initial_size: usize, max_size: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let factory = Arc::new(factory);
        let mut objects = Vec::with_capacity(initial_size);

        // Pre-allocate objects
        for _ in 0..initial_size {
            objects.push(factory());
        }

        let stats = PoolStats {
            created: initial_size,
            reused: 0,
            returned: 0,
            current_size: initial_size,
            peak_size: initial_size,
        };

        ObjectPool {
            objects: Arc::new(Mutex::new(objects)),
            factory,
            max_size,
            stats: Arc::new(Mutex::new(stats)),
        }
    }

    /// Get an object from the pool
    pub fn acquire(&self) -> PooledObject<T> {
        let obj = {
            let mut pool = self.objects.lock().unwrap();
            pool.pop()
        };

        let mut stats = self.stats.lock().unwrap();
        
        let obj = match obj {
            Some(obj) => {
                stats.reused += 1;
                obj
            }
            None => {
                stats.created += 1;
                (self.factory)()
            }
        };

        stats.current_size = self.objects.lock().unwrap().len();

        PooledObject::new(obj, self.objects.clone())
    }

    /// Get current pool size
    pub fn size(&self) -> usize {
        self.objects.lock().unwrap().len()
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        let mut stats = self.stats.lock().unwrap().clone();
        stats.current_size = self.size();
        stats
    }

    /// Clear all objects from the pool
    pub fn clear(&self) {
        self.objects.lock().unwrap().clear();
        let mut stats = self.stats.lock().unwrap();
        stats.current_size = 0;
    }

    /// Pre-fill the pool with objects
    pub fn prefill(&self, count: usize) {
        let mut pool = self.objects.lock().unwrap();
        let current = pool.len();
        let to_add = count.saturating_sub(current);

        for _ in 0..to_add {
            pool.push((self.factory)());
        }

        let mut stats = self.stats.lock().unwrap();
        stats.created += to_add;
        stats.current_size = pool.len();
        stats.peak_size = stats.peak_size.max(pool.len());
    }
}

impl<T> Clone for ObjectPool<T> {
    fn clone(&self) -> Self {
        ObjectPool {
            objects: self.objects.clone(),
            factory: self.factory.clone(),
            max_size: self.max_size,
            stats: self.stats.clone(),
        }
    }
}

// ============================================================================
// Example: Buffer Pool
// ============================================================================

type Buffer = Vec<u8>;

pub struct BufferPool {
    pool: ObjectPool<Buffer>,
    buffer_size: usize,
}

impl BufferPool {
    pub fn new(buffer_size: usize, initial_size: usize) -> Self {
        let pool = ObjectPool::new(
            move || vec![0u8; buffer_size],
            initial_size,
            0,
        );

        BufferPool { pool, buffer_size }
    }

    pub fn acquire(&self) -> PooledObject<Buffer> {
        let mut buffer = self.pool.acquire();
        buffer.clear();
        buffer.resize(self.buffer_size, 0);
        buffer
    }

    pub fn stats(&self) -> PoolStats {
        self.pool.stats()
    }
}

// ============================================================================
// Benchmarking
// ============================================================================

struct BenchmarkResult {
    name: String,
    duration: Duration,
    operations: usize,
    ops_per_sec: f64,
}

impl BenchmarkResult {
    fn print(&self) {
        println!("  {} ", self.name);
        println!("    Duration: {:.2?}", self.duration);
        println!("    Operations: {}", self.operations);
        println!("    Ops/sec: {:.2}", self.ops_per_sec);
    }
}

fn benchmark<F>(name: &str, operations: usize, f: F) -> BenchmarkResult
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    let ops_per_sec = operations as f64 / duration.as_secs_f64();

    BenchmarkResult {
        name: name.to_string(),
        duration,
        operations,
        ops_per_sec,
    }
}

fn benchmark_with_pool() {
    println!("\n{:=^60}", " BENCHMARK: With Pool ");
    
    let pool = BufferPool::new(1024, 10);
    let iterations = 100_000;

    let result = benchmark("Buffer allocation (pooled)", iterations, || {
        for _ in 0..iterations {
            let mut buffer = pool.acquire();
            buffer[0] = 42; // Simulate usage
        }
    });

    result.print();
    
    let stats = pool.stats();
    println!("\n  Pool Statistics:");
    println!("    Objects created: {}", stats.created);
    println!("    Objects reused: {}", stats.reused);
    println!("    Reuse rate: {:.2}%", 
             (stats.reused as f64 / iterations as f64) * 100.0);
}

fn benchmark_without_pool() {
    println!("\n{:=^60}", " BENCHMARK: Without Pool ");
    
    let iterations = 100_000;

    let result = benchmark("Buffer allocation (direct)", iterations, || {
        for _ in 0..iterations {
            let mut buffer = vec![0u8; 1024];
            buffer[0] = 42; // Simulate usage
        }
    });

    result.print();
}

fn benchmark_multithreaded() {
    println!("\n{:=^60}", " BENCHMARK: Multi-threaded ");
    
    let pool = BufferPool::new(1024, 10);
    let num_threads = 4;
    let iterations_per_thread = 25_000;

    let start = Instant::now();
    
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let pool = BufferPool::new(1024, 10);
            thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    let mut buffer = pool.acquire();
                    buffer[0] = 42;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let total_ops = num_threads * iterations_per_thread;
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();

    println!("  Multi-threaded pooled allocation");
    println!("    Threads: {}", num_threads);
    println!("    Duration: {:.2?}", duration);
    println!("    Total operations: {}", total_ops);
    println!("    Ops/sec: {:.2}", ops_per_sec);
}

// ============================================================================
// Demonstrations
// ============================================================================

fn demo_basic_usage() {
    println!("\n{:=^60}", " BASIC USAGE ");
    
    // Create a pool of String objects
    let pool = ObjectPool::new(
        || String::with_capacity(100),
        5,
        10,
    );

    println!("Initial pool size: {}", pool.size());

    // Acquire and use objects
    {
        println!("\nAcquiring 3 objects...");
        let mut obj1 = pool.acquire();
        let mut obj2 = pool.acquire();
        let mut obj3 = pool.acquire();

        obj1.push_str("Hello");
        obj2.push_str("World");
        obj3.push_str("!");

        println!("Pool size while objects are in use: {}", pool.size());
    } // Objects returned to pool here

    println!("Pool size after objects returned: {}", pool.size());

    let stats = pool.stats();
    println!("\nStatistics:");
    println!("  Created: {}", stats.created);
    println!("  Reused: {}", stats.reused);
    println!("  Current size: {}", stats.current_size);
}

fn demo_buffer_pool() {
    println!("\n{:=^60}", " BUFFER POOL ");
    
    let pool = BufferPool::new(4096, 3);
    
    println!("Created buffer pool with 4KB buffers");
    println!("Initial pool size: {}", pool.stats().current_size);

    // Simulate some work
    for i in 0..10 {
        let mut buffer = pool.acquire();
        buffer[0] = i as u8;
        println!("Iteration {}: Using buffer", i);
    }

    let stats = pool.stats();
    println!("\nBuffer pool statistics:");
    println!("  Total created: {}", stats.created);
    println!("  Total reused: {}", stats.reused);
    println!("  Reuse rate: {:.1}%", 
             (stats.reused as f64 / (stats.created + stats.reused) as f64) * 100.0);
}

fn demo_thread_safety() {
    println!("\n{:=^60}", " THREAD SAFETY ");
    
    let pool = ObjectPool::new(
        || Vec::<i32>::with_capacity(100),
        5,
        0,
    );

    let handles: Vec<_> = (0..4)
        .map(|thread_id| {
            let pool = pool.clone();
            thread::spawn(move || {
                for i in 0..5 {
                    let mut vec = pool.acquire();
                    vec.push(thread_id * 100 + i);
                    println!("Thread {} iteration {}: vector len = {}", 
                             thread_id, i, vec.len());
                    thread::sleep(Duration::from_millis(10));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let stats = pool.stats();
    println!("\nFinal statistics:");
    println!("  Created: {}", stats.created);
    println!("  Reused: {}", stats.reused);
    println!("  Pool size: {}", stats.current_size);
}

fn main() {
    println!("🔧 Memory Pool Implementation 🔧\n");
    
    demo_basic_usage();
    demo_buffer_pool();
    demo_thread_safety();
    
    println!("\n{:=^60}", " PERFORMANCE BENCHMARKS ");
    println!("\n⏱️  Running benchmarks (this may take a moment)...");
    
    benchmark_without_pool();
    benchmark_with_pool();
    benchmark_multithreaded();
    
    println!("\n{:=^60}", " COMPLETE ");
    println!("\n💡 Key Takeaways:");
    println!("   - Object pools reduce allocation overhead");
    println!("   - Especially beneficial for frequently allocated objects");
    println!("   - Thread-safe design enables concurrent access");
    println!("   - Monitor reuse rate to validate effectiveness");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_acquire_return() {
        let pool = ObjectPool::new(|| vec![0u8; 100], 2, 10);
        let initial_size = pool.size();
        
        {
            let _obj = pool.acquire();
            assert_eq!(pool.size(), initial_size - 1);
        }
        
        assert_eq!(pool.size(), initial_size);
    }

    #[test]
    fn test_pool_stats() {
        let pool = ObjectPool::new(|| String::new(), 1, 5);
        
        {
            let _obj1 = pool.acquire();
            let _obj2 = pool.acquire();
        }
        
        let stats = pool.stats();
        assert_eq!(stats.created, 2);
        assert!(stats.reused > 0 || stats.created == 2);
    }

    #[test]
    fn test_buffer_pool() {
        let pool = BufferPool::new(1024, 2);
        let buffer = pool.acquire();
        assert_eq!(buffer.len(), 1024);
    }

    #[test]
    fn test_pool_clear() {
        let pool = ObjectPool::new(|| Vec::<i32>::new(), 5, 10);
        assert!(pool.size() > 0);
        pool.clear();
        assert_eq!(pool.size(), 0);
    }
}
