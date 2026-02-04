# Rust WebSocket Chat Application

A real-time chat application built with Rust, featuring WebSocket communication, multiple chat rooms, private messaging, and message persistence.

## Features

- 🔌 **WebSocket Server** - Real-time bidirectional communication with tokio-tungstenite
- 🏠 **Multiple Chat Rooms** - Create and join different chat rooms
- ⚡ **Async Everything** - Built on Tokio for high-performance async I/O
- 💾 **Message Persistence** - SQLite database stores all messages
- 🔒 **Private Messaging** - Send direct messages to specific users
- 🌐 **Web Client** - Clean HTML/CSS/JS interface
- 📊 **Room Management** - Dynamic room creation and listing
- 👥 **User Management** - Username system with connection tracking

## Tech Stack

- **Server**: Tokio + tokio-tungstenite
- **Database**: SQLite with SQLx
- **Concurrency**: DashMap for thread-safe state
- **Serialization**: Serde + serde_json
- **Client**: Vanilla HTML/CSS/JavaScript

## Project Structure

```
chat-application/
├── src/
│   ├── main.rs          # Entry point and connection handling
│   ├── server.rs        # Chat server logic
│   ├── models.rs        # Message and client models
│   └── db.rs           # Database operations
├── client/
│   ├── index.html      # Web client UI
│   ├── style.css       # Styling
│   └── app.js          # WebSocket client logic
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Modern web browser

### Installation & Running

1. Navigate to the project:
```bash
cd chat-application
```

2. Build and run the server:
```bash
cargo build --release
cargo run
```

The server will start on `ws://127.0.0.1:9001`

3. Open the client:
```bash
# Open in your browser
open client/index.html
# or on Linux
xdg-open client/index.html
```

## Usage

### Connecting

1. Open `client/index.html` in your browser
2. Click "Connect" button
3. Enter your username when prompted

### Commands

The chat supports several commands:

- `/nick [username]` - Set or change your username
- `/join [room]` - Join a chat room
- `/pm [username] [message]` - Send a private message
- `/rooms` - List all available rooms

### Default Rooms

The application comes with three pre-configured rooms:
- **general** - General discussion
- **random** - Random topics
- **tech** - Technology discussions

### Sending Messages

1. Join a room by clicking on it in the sidebar or using `/join [room]`
2. Type your message in the input box
3. Press Enter or click Send

### Private Messages

Send a private message with:
```
/pm username Hello, this is private!
```

## Architecture

### WebSocket Communication

The server uses `tokio-tungstenite` for WebSocket handling:
- Each client connection runs in its own Tokio task
- Messages are sent through unbounded channels for efficient async communication
- Split stream architecture for concurrent reading and writing

### Message Types

**Client → Server:**
```rust
SetUsername { username: String }
JoinRoom { room: String }
SendMessage { content: String }
PrivateMessage { to: String, content: String }
ListRooms
```

**Server → Client:**
```rust
Message { username, content, timestamp, room }
PrivateMessage { from, content, timestamp }
SystemMessage { content }
UserJoined { username, room }
UserLeft { username, room }
```

### State Management

- **DashMap** for thread-safe concurrent access to:
  - Connected clients
  - Usernames
  - Room memberships
- Lock-free reads and writes for high performance

### Database Schema

**Messages Table:**
```sql
CREATE TABLE messages (
    id TEXT PRIMARY KEY,
    room TEXT NOT NULL,
    username TEXT NOT NULL,
    content TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    is_private BOOLEAN NOT NULL
)
```

**Rooms Table:**
```sql
CREATE TABLE rooms (
    name TEXT PRIMARY KEY,
    created_at TEXT NOT NULL
)
```

## API Reference

### WebSocket Protocol

Connect to: `ws://127.0.0.1:9001`

Send JSON messages in this format:
```json
{
  "type": "SendMessage",
  "content": "Hello, world!"
}
```

Receive messages:
```json
{
  "type": "Message",
  "username": "john",
  "content": "Hello, world!",
  "timestamp": "2024-01-01T12:00:00Z",
  "room": "general"
}
```

## Development

### Run with logging:
```bash
RUST_LOG=info cargo run
```

### Run tests:
```bash
cargo test
```

### Database Location

The SQLite database is created as `chat.db` in the project root.

View messages:
```bash
sqlite3 chat.db "SELECT * FROM messages ORDER BY timestamp DESC LIMIT 10;"
```

## Features Explained

### Async Connection Handling

Each WebSocket connection is handled asynchronously:
```rust
while let Ok((stream, _)) = listener.accept().await {
    tokio::spawn(async move {
        handle_connection(stream, server).await
    });
}
```

### Message Broadcasting

Messages are broadcast to all clients in a room efficiently:
```rust
for client_id in room_clients {
    sender.send(message.clone()).await;
}
```

### Graceful Disconnection

When a client disconnects:
1. Removes from all rooms
2. Cleans up username mapping
3. Notifies other users
4. Closes WebSocket connection

## Performance

- **Concurrent Connections**: Handles thousands of simultaneous connections
- **Message Throughput**: Lock-free data structures for minimal latency
- **Memory Efficient**: Shared state with Arc and streaming I/O

## Security Considerations

This is a learning project. For production use, add:
- Authentication and authorization
- Input validation and sanitization
- Rate limiting
- TLS/WSS encryption
- CORS policies

## Extending the Project

Ideas for enhancements:
- User authentication with passwords
- Persistent user profiles
- File sharing
- Emoji reactions
- Message editing/deletion
- Typing indicators
- Read receipts
- Voice/video chat

## Troubleshooting

**Can't connect to WebSocket:**
- Ensure server is running (`cargo run`)
- Check firewall settings
- Verify port 9001 is available

**Messages not appearing:**
- Check browser console for errors
- Verify you've joined a room
- Ensure username is set

## License

MIT License - Use for learning and personal projects.

## Learning Resources

This project demonstrates:
- Async/await in Rust
- WebSocket protocol
- Tokio runtime
- SQLx database operations
- Concurrent state management
- Real-time communication patterns
