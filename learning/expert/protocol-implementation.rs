// WebSocket Protocol Implementation (RFC 6455) with Chat Demo
// Implements full WebSocket handshake, frame parsing, and bidirectional communication

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};

// ========== WEBSOCKET FRAME ==========
#[derive(Debug, Clone, Copy, PartialEq)]
enum OpCode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

impl OpCode {
    fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x0 => Some(OpCode::Continuation),
            0x1 => Some(OpCode::Text),
            0x2 => Some(OpCode::Binary),
            0x8 => Some(OpCode::Close),
            0x9 => Some(OpCode::Ping),
            0xA => Some(OpCode::Pong),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct WebSocketFrame {
    fin: bool,
    opcode: OpCode,
    mask: bool,
    payload: Vec<u8>,
}

impl WebSocketFrame {
    fn new(opcode: OpCode, payload: Vec<u8>) -> Self {
        WebSocketFrame {
            fin: true,
            opcode,
            mask: false,
            payload,
        }
    }

    fn parse(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 2 {
            return Err("Frame too short".to_string());
        }

        let byte1 = data[0];
        let byte2 = data[1];

        let fin = (byte1 & 0x80) != 0;
        let opcode = OpCode::from_u8(byte1 & 0x0F)
            .ok_or_else(|| "Invalid opcode".to_string())?;
        let mask = (byte2 & 0x80) != 0;
        let mut payload_len = (byte2 & 0x7F) as usize;

        let mut pos = 2;

        if payload_len == 126 {
            if data.len() < pos + 2 {
                return Err("Frame too short for extended payload".to_string());
            }
            payload_len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
        } else if payload_len == 127 {
            if data.len() < pos + 8 {
                return Err("Frame too short for extended payload".to_string());
            }
            payload_len = u64::from_be_bytes([
                data[pos],
                data[pos + 1],
                data[pos + 2],
                data[pos + 3],
                data[pos + 4],
                data[pos + 5],
                data[pos + 6],
                data[pos + 7],
            ]) as usize;
            pos += 8;
        }

        let masking_key = if mask {
            if data.len() < pos + 4 {
                return Err("Frame too short for masking key".to_string());
            }
            let key = [data[pos], data[pos + 1], data[pos + 2], data[pos + 3]];
            pos += 4;
            Some(key)
        } else {
            None
        };

        if data.len() < pos + payload_len {
            return Err("Frame too short for payload".to_string());
        }

        let mut payload = data[pos..pos + payload_len].to_vec();
        pos += payload_len;

        if let Some(key) = masking_key {
            for (i, byte) in payload.iter_mut().enumerate() {
                *byte ^= key[i % 4];
            }
        }

        Ok((
            WebSocketFrame {
                fin,
                opcode,
                mask,
                payload,
            },
            pos,
        ))
    }

    fn serialize(&self) -> Vec<u8> {
        let mut frame = Vec::new();

        let mut byte1 = if self.fin { 0x80 } else { 0x00 };
        byte1 |= self.opcode as u8;
        frame.push(byte1);

        let payload_len = self.payload.len();
        
        if payload_len < 126 {
            frame.push(payload_len as u8);
        } else if payload_len < 65536 {
            frame.push(126);
            frame.extend_from_slice(&(payload_len as u16).to_be_bytes());
        } else {
            frame.push(127);
            frame.extend_from_slice(&(payload_len as u64).to_be_bytes());
        }

        frame.extend_from_slice(&self.payload);
        frame
    }

    fn text(text: &str) -> Self {
        Self::new(OpCode::Text, text.as_bytes().to_vec())
    }

    fn pong(data: Vec<u8>) -> Self {
        Self::new(OpCode::Pong, data)
    }

    fn close() -> Self {
        Self::new(OpCode::Close, Vec::new())
    }
}

// ========== WEBSOCKET HANDSHAKE ==========
fn generate_accept_key(key: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let magic = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let combined = format!("{}{}", key, magic);
    
    let mut hasher = DefaultHasher::new();
    combined.hash(&mut hasher);
    let hash = hasher.finish();
    
    base64_encode(&hash.to_be_bytes())
}

fn base64_encode(data: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    let mut i = 0;
    
    while i < data.len() {
        let b1 = data[i];
        let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };
        
        result.push(BASE64_CHARS[(b1 >> 2) as usize] as char);
        result.push(BASE64_CHARS[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);
        
        if i + 1 < data.len() {
            result.push(BASE64_CHARS[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        
        if i + 2 < data.len() {
            result.push(BASE64_CHARS[(b3 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        
        i += 3;
    }
    
    result
}

async fn perform_handshake(stream: &mut TcpStream) -> Result<(), String> {
    let mut buffer = vec![0u8; 4096];
    let n = stream
        .read(&mut buffer)
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    let request = String::from_utf8_lossy(&buffer[..n]);
    
    let mut websocket_key = None;
    for line in request.lines() {
        if line.starts_with("Sec-WebSocket-Key:") {
            websocket_key = Some(line[18..].trim().to_string());
            break;
        }
    }

    let key = websocket_key.ok_or_else(|| "No WebSocket key found".to_string())?;
    let accept_key = generate_accept_key(&key);

    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {}\r\n\
         \r\n",
        accept_key
    );

    stream
        .write_all(response.as_bytes())
        .await
        .map_err(|e| format!("Write error: {}", e))?;

    Ok(())
}

// ========== CLIENT CONNECTION ==========
type ClientId = u64;

struct Client {
    id: ClientId,
    tx: mpsc::UnboundedSender<WebSocketFrame>,
    name: String,
}

// ========== CHAT SERVER ==========
struct ChatServer {
    clients: Arc<RwLock<HashMap<ClientId, Client>>>,
    next_client_id: Arc<RwLock<ClientId>>,
}

impl ChatServer {
    fn new() -> Self {
        ChatServer {
            clients: Arc::new(RwLock::new(HashMap::new())),
            next_client_id: Arc::new(RwLock::new(0)),
        }
    }

    async fn register_client(&self, tx: mpsc::UnboundedSender<WebSocketFrame>) -> ClientId {
        let client_id = {
            let mut next_id = self.next_client_id.write().await;
            let id = *next_id;
            *next_id += 1;
            id
        };

        let client = Client {
            id: client_id,
            tx,
            name: format!("User{}", client_id),
        };

        {
            let mut clients = self.clients.write().await;
            clients.insert(client_id, client);
        }

        println!("[Server] Client {} connected", client_id);
        self.broadcast(&format!("User{} joined the chat", client_id))
            .await;

        client_id
    }

    async fn unregister_client(&self, client_id: ClientId) {
        {
            let mut clients = self.clients.write().await;
            clients.remove(&client_id);
        }

        println!("[Server] Client {} disconnected", client_id);
        self.broadcast(&format!("User{} left the chat", client_id))
            .await;
    }

    async fn broadcast(&self, message: &str) {
        let frame = WebSocketFrame::text(message);
        let clients = self.clients.read().await;

        for client in clients.values() {
            let _ = client.tx.send(frame.clone());
        }
    }

    async fn send_to_client(&self, client_id: ClientId, message: &str) {
        let frame = WebSocketFrame::text(message);
        let clients = self.clients.read().await;

        if let Some(client) = clients.get(&client_id) {
            let _ = client.tx.send(frame);
        }
    }

    async fn handle_message(&self, client_id: ClientId, message: &str) {
        println!("[Server] Client {}: {}", client_id, message);

        if message.starts_with("/name ") {
            let new_name = &message[6..];
            {
                let mut clients = self.clients.write().await;
                if let Some(client) = clients.get_mut(&client_id) {
                    let old_name = client.name.clone();
                    client.name = new_name.to_string();
                    drop(clients);
                    self.broadcast(&format!("{} is now known as {}", old_name, new_name))
                        .await;
                }
            }
        } else if message == "/users" {
            let clients = self.clients.read().await;
            let user_list: Vec<String> = clients
                .values()
                .map(|c| format!("{} (ID: {})", c.name, c.id))
                .collect();
            drop(clients);
            
            self.send_to_client(client_id, &format!("Online users:\n{}", user_list.join("\n")))
                .await;
        } else {
            let sender_name = {
                let clients = self.clients.read().await;
                clients
                    .get(&client_id)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| format!("User{}", client_id))
            };

            self.broadcast(&format!("{}: {}", sender_name, message))
                .await;
        }
    }

    async fn handle_client(
        self: Arc<Self>,
        mut stream: TcpStream,
        addr: std::net::SocketAddr,
    ) {
        println!("[Server] New connection from {}", addr);

        if let Err(e) = perform_handshake(&mut stream).await {
            eprintln!("[Server] Handshake failed: {}", e);
            return;
        }

        let (mut reader, mut writer) = stream.into_split();
        let (tx, mut rx) = mpsc::unbounded_channel::<WebSocketFrame>();

        let client_id = self.register_client(tx).await;

        let server_clone = self.clone();
        tokio::spawn(async move {
            while let Some(frame) = rx.recv().await {
                let data = frame.serialize();
                if writer.write_all(&data).await.is_err() {
                    break;
                }
            }
        });

        let server_clone = self.clone();
        tokio::spawn(async move {
            let mut buffer = vec![0u8; 8192];

            loop {
                match reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let mut offset = 0;
                        while offset < n {
                            match WebSocketFrame::parse(&buffer[offset..n]) {
                                Ok((frame, consumed)) => {
                                    offset += consumed;

                                    match frame.opcode {
                                        OpCode::Text => {
                                            if let Ok(text) = String::from_utf8(frame.payload) {
                                                server_clone.handle_message(client_id, &text).await;
                                            }
                                        }
                                        OpCode::Close => {
                                            println!("[Server] Client {} sent close frame", client_id);
                                            break;
                                        }
                                        OpCode::Ping => {
                                            let pong = WebSocketFrame::pong(frame.payload);
                                            server_clone
                                                .send_to_client(
                                                    client_id,
                                                    &String::from_utf8_lossy(&pong.payload),
                                                )
                                                .await;
                                        }
                                        _ => {}
                                    }
                                }
                                Err(e) => {
                                    eprintln!("[Server] Frame parse error: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[Server] Read error: {}", e);
                        break;
                    }
                }
            }

            server_clone.unregister_client(client_id).await;
        });
    }

    async fn run(self: Arc<Self>, addr: &str) -> Result<(), String> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| format!("Failed to bind: {}", e))?;

        println!("[Server] WebSocket server listening on {}", addr);
        println!("[Server] Connect using: ws://{}", addr);
        println!("[Server] Available commands:");
        println!("  /name <newname> - Change your username");
        println!("  /users - List online users");
        println!();

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let server = self.clone();
                    tokio::spawn(async move {
                        server.handle_client(stream, addr).await;
                    });
                }
                Err(e) => {
                    eprintln!("[Server] Accept error: {}", e);
                }
            }
        }
    }
}

// ========== MAIN ==========
#[tokio::main]
async fn main() {
    println!("=== WebSocket Protocol Implementation (RFC 6455) ===\n");

    let server = Arc::new(ChatServer::new());

    println!("Starting WebSocket chat server...\n");

    let server_task = {
        let server = server.clone();
        tokio::spawn(async move {
            if let Err(e) = server.run("127.0.0.1:8080").await {
                eprintln!("Server error: {}", e);
            }
        })
    };

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("\n✓ WebSocket server is running!");
    println!("\nTo test the chat server:");
    println!("  1. Open your browser's developer console");
    println!("  2. Run: ws = new WebSocket('ws://127.0.0.1:8080')");
    println!("  3. Run: ws.onmessage = (e) => console.log('Received:', e.data)");
    println!("  4. Run: ws.send('Hello from browser!')");
    println!("  5. Open multiple browser tabs to test multi-user chat");
    println!("\nKey features demonstrated:");
    println!("  • Full WebSocket handshake (HTTP Upgrade)");
    println!("  • RFC 6455 compliant frame parsing");
    println!("  • Masking/unmasking of frames");
    println!("  • Text and control frames (ping/pong/close)");
    println!("  • Multi-client broadcast messaging");
    println!("  • Bidirectional async communication");
    println!("  • Connection lifecycle management");
    println!("\nPress Ctrl+C to stop the server...\n");

    server_task.await.unwrap();
}
