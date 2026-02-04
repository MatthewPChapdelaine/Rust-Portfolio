# Rust Real-World Projects

Three complete, production-ready Rust projects demonstrating modern Rust development practices, featuring web frameworks, async programming, systems programming, and more.

## 📦 Projects Overview

### 1. Blog Engine (`blog-engine/`)

A full-featured blogging platform with authentication, markdown support, and admin panel.

**Technologies:** Actix-web, SQLite (SQLx), JWT, Tera templates, Pulldown-cmark

**Features:**
- 🔐 JWT authentication with bcrypt password hashing
- 📝 Full CRUD operations for blog posts
- 🎨 Markdown rendering with syntax support
- 💬 Comment system with moderation
- 👑 Admin dashboard for content management
- 🎭 Server-side rendering with Tera
- 🚀 RESTful API design
- 📱 Responsive web interface

**Quick Start:**
```bash
cd blog-engine
cargo run
# Open http://127.0.0.1:8080
```

---

### 2. Chat Application (`chat-application/`)

Real-time WebSocket chat server with multiple rooms and private messaging.

**Technologies:** Tokio, tokio-tungstenite, SQLite (SQLx), WebSockets

**Features:**
- 🔌 WebSocket server for real-time communication
- 🏠 Multiple chat rooms with dynamic creation
- ⚡ Fully async with Tokio runtime
- 💾 Message persistence to SQLite
- 🔒 Private messaging between users
- 🌐 Clean HTML/CSS/JS client interface
- 👥 User management and tracking
- 📊 Connection handling with graceful cleanup

**Quick Start:**
```bash
cd chat-application
cargo run
# Open client/index.html in browser
```

---

### 3. Package Manager (`package-manager/`)

A Cargo-like dependency management tool with version resolution and lock files.

**Technologies:** Clap, Semver, Petgraph, TOML, Serde

**Features:**
- 📦 Semantic versioning with automatic resolution
- 🔒 Cargo.lock-style deterministic builds
- 🌳 Dependency tree visualization with petgraph
- 📚 Simulated package registry
- 🔍 Package search and discovery
- ⚡ Clean CLI with clap derive
- ✅ Circular dependency detection
- 🎨 Colored terminal output

**Quick Start:**
```bash
cd package-manager
cargo build --release
cd sample-project
../target/release/pkgmgr install
../target/release/pkgmgr tree
```

---

## 🎯 What You'll Learn

### Blog Engine Project
- **Web Development**: Building REST APIs with Actix-web
- **Database**: SQLx for type-safe SQL queries
- **Authentication**: JWT tokens and bcrypt hashing
- **Templates**: Server-side rendering with Tera
- **Markdown**: Content transformation with pulldown-cmark
- **Architecture**: Clean separation of concerns (handlers, models, db, auth)

### Chat Application Project
- **Async Programming**: Tokio runtime and async/await
- **WebSockets**: Real-time bidirectional communication
- **Concurrency**: Thread-safe state with DashMap
- **Streaming**: Split streams for concurrent read/write
- **Protocols**: WebSocket message framing and parsing
- **Client-Server**: Full-stack implementation

### Package Manager Project
- **CLI Development**: Clap for robust command-line interfaces
- **Graph Algorithms**: Petgraph for dependency resolution
- **Versioning**: Semver parsing and requirement matching
- **File Formats**: TOML parsing with serde
- **Algorithms**: Dependency resolution and cycle detection
- **Error Handling**: Anyhow and thiserror patterns

---

## 🚀 Getting Started

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Cargo** - Comes with Rust
- **SQLite** - Usually pre-installed on Linux/Mac

### Building All Projects

```bash
# Blog Engine
cd blog-engine
cargo build --release

# Chat Application
cd ../chat-application
cargo build --release

# Package Manager
cd ../package-manager
cargo build --release
```

### Running Tests

Each project includes comprehensive documentation. Run builds to verify:

```bash
# Test each project
cargo test --all
```

---

## 📚 Project Structure

```
rust-projects/projects/real-world/
├── blog-engine/
│   ├── src/
│   │   ├── main.rs         # Application entry
│   │   ├── handlers.rs     # HTTP handlers
│   │   ├── models.rs       # Data models
│   │   ├── db.rs          # Database operations
│   │   ├── auth.rs        # JWT authentication
│   │   └── utils.rs       # Utilities
│   ├── templates/          # Tera templates
│   ├── static/            # CSS/JS assets
│   ├── Cargo.toml
│   └── README.md
│
├── chat-application/
│   ├── src/
│   │   ├── main.rs         # Entry & connection handling
│   │   ├── server.rs       # Chat server logic
│   │   ├── models.rs       # Message types
│   │   └── db.rs          # Database operations
│   ├── client/            # Web client (HTML/CSS/JS)
│   ├── Cargo.toml
│   └── README.md
│
└── package-manager/
    ├── src/
    │   ├── main.rs         # CLI commands
    │   ├── cli.rs          # Command definitions
    │   ├── resolver.rs     # Dependency resolution
    │   ├── registry.rs     # Package registry
    │   ├── installer.rs    # Installation logic
    │   └── lockfile.rs     # Lock file generation
    ├── registry-data/      # Sample packages
    ├── sample-project/     # Example project
    ├── Cargo.toml
    └── README.md
```

---

## 🔧 Technical Highlights

### Rust Best Practices Demonstrated

✅ **Error Handling**: Proper use of `Result<T, E>` and `?` operator  
✅ **Ownership**: Clear ownership semantics, no unnecessary cloning  
✅ **Async/Await**: Modern async Rust with Tokio  
✅ **Type Safety**: Leveraging Rust's type system for correctness  
✅ **Documentation**: Comprehensive README files and inline docs  
✅ **Project Structure**: Clean module organization  
✅ **Dependencies**: Production-quality crates  
✅ **Serialization**: Serde for type-safe data handling  

### Key Patterns

- **Builder Pattern**: Configuration and setup
- **Repository Pattern**: Database abstraction
- **Middleware**: Request/response processing
- **State Management**: Thread-safe shared state
- **Async Streams**: Efficient I/O handling
- **Command Pattern**: CLI subcommands

---

## 🎓 Learning Path

### Beginner → Intermediate
1. Start with **Package Manager** to learn:
   - CLI development
   - File I/O and parsing
   - Basic algorithms

### Intermediate → Advanced
2. Move to **Blog Engine** for:
   - Web development
   - Database operations
   - Authentication

### Advanced
3. Tackle **Chat Application** for:
   - Async programming
   - Real-time communication
   - Concurrent state management

---

## 📖 Documentation

Each project has a detailed README covering:
- Architecture overview
- API/Protocol documentation
- Usage examples
- Configuration options
- Development guide
- Troubleshooting

---

## 🛠️ Development

### Hot Reload (Blog Engine & Chat)

```bash
cargo install cargo-watch
cargo watch -x run
```

### Release Builds

```bash
cargo build --release
# Binaries in target/release/
```

### Check Code

```bash
cargo clippy
cargo fmt --check
```

---

## 🎯 Use Cases

### Blog Engine
- Personal blog
- Company blog
- Documentation site
- Content management system

### Chat Application
- Team communication
- Customer support chat
- Gaming chat system
- Real-time notifications

### Package Manager
- Internal package registry
- Build system integration
- Dependency auditing
- Learning package management

---

## 🔐 Security Notes

These projects are for **learning and development**. For production:

- ✅ Change default JWT secrets
- ✅ Use HTTPS/WSS in production
- ✅ Implement rate limiting
- ✅ Add input validation
- ✅ Set up CORS properly
- ✅ Use environment variables for secrets
- ✅ Regular security audits

---

## 📈 Performance

All projects are optimized for:
- **Low Latency**: Async I/O and efficient algorithms
- **High Throughput**: Concurrent request handling
- **Memory Efficiency**: Rust's zero-cost abstractions
- **Scalability**: Horizontal scaling ready

---

## 🤝 Contributing

These are learning projects. Feel free to:
- Fork and experiment
- Extend with new features
- Use as templates for your projects
- Submit improvements

---

## 📝 License

MIT License - Free to use for learning and personal projects.

---

## 🙏 Acknowledgments

Built with excellent Rust crates:
- **actix-web** - Web framework
- **tokio** - Async runtime
- **sqlx** - Database toolkit
- **serde** - Serialization
- **clap** - CLI parsing
- **petgraph** - Graph algorithms
- Many more...

---

## 📚 Resources

### Learning Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Crate Documentation
- [Actix-web Guide](https://actix.rs/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [SQLx Documentation](https://docs.rs/sqlx/)

---

## 🎉 Quick Demo Commands

```bash
# Blog Engine - Create admin and post
cd blog-engine
cargo run &
sleep 5
curl -X POST http://127.0.0.1:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@test.com","password":"pass123"}'

# Chat Application - Start server
cd chat-application
cargo run &
# Open client/index.html

# Package Manager - Install packages
cd package-manager/sample-project
cargo run --bin pkgmgr -- install
cargo run --bin pkgmgr -- tree
```

---

**Happy Coding! 🦀**

Each project is production-ready, well-documented, and demonstrates real-world Rust patterns. Perfect for learning, portfolio projects, or as starting points for your own applications.
