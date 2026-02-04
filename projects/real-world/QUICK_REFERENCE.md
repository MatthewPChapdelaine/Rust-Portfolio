# Quick Reference Guide

## 🚀 Getting Started

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

---

## 📦 Project 1: Blog Engine

### Directory
```bash
cd /home/matthew/repos/Programming_Repos/rust-projects/projects/real-world/blog-engine
```

### Build & Run
```bash
cargo build --release    # Build optimized binary
cargo run                # Run development server
./demo.sh               # Run automated demo
```

### Key Commands
```bash
# Register user
curl -X POST http://127.0.0.1:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"user","email":"user@example.com","password":"pass123"}'

# Login
curl -X POST http://127.0.0.1:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"user","password":"pass123"}'

# Create post (with JWT token)
curl -X POST http://127.0.0.1:8080/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"title":"Post","summary":"Summary","content":"# Content","published":true}'
```

### URLs
- Homepage: http://127.0.0.1:8080/
- Admin Panel: http://127.0.0.1:8080/admin
- API Docs: See README.md

---

## 💬 Project 2: Chat Application

### Directory
```bash
cd /home/matthew/repos/Programming_Repos/rust-projects/projects/real-world/chat-application
```

### Build & Run
```bash
cargo build --release    # Build optimized binary
cargo run                # Start WebSocket server
```

### Connect Client
```bash
# Open in browser
firefox client/index.html
# or
google-chrome client/index.html
```

### Chat Commands
```
/nick username          - Set your username
/join roomname         - Join a chat room
/pm username message   - Private message
/rooms                 - List all rooms
```

### Server
- WebSocket: ws://127.0.0.1:9001
- Default rooms: general, random, tech

---

## 📚 Project 3: Package Manager

### Directory
```bash
cd /home/matthew/repos/Programming_Repos/rust-projects/projects/real-world/package-manager
```

### Build
```bash
cargo build --release
# Binary: ./target/release/pkgmgr
```

### Commands
```bash
# Initialize new package
./target/release/pkgmgr init my-project

# Install dependencies
./target/release/pkgmgr install

# Update dependencies
./target/release/pkgmgr update

# View dependency tree
./target/release/pkgmgr tree

# List packages
./target/release/pkgmgr registry list

# Search packages
./target/release/pkgmgr registry search web

# Package info
./target/release/pkgmgr registry info tokio

# Run full demo
./demo.sh
```

### Package.toml Format
```toml
[package]
name = "my-app"
version = "0.1.0"
authors = ["Name <email@example.com>"]

[dependencies]
serde = "^1.0"    # Caret: >=1.0.0, <2.0.0
tokio = "~1.35"   # Tilde: >=1.35.0, <1.36.0
actix-web = "4.4.0"  # Exact version
```

---

## 🔧 Common Tasks

### Clean Build Artifacts
```bash
cargo clean              # Remove target/ directory
```

### Check Code
```bash
cargo check              # Fast compile check
cargo clippy             # Linting
cargo fmt                # Format code
```

### Run Tests
```bash
cargo test               # Run all tests
cargo test --release     # Release mode tests
```

### Update Dependencies
```bash
cargo update             # Update Cargo.lock
```

### View Documentation
```bash
cargo doc --open         # Generate & open docs
```

---

## 📁 Project Structure

```
real-world/
├── README.md                  # Main documentation
├── PROJECT_SUMMARY.md         # Completion summary
├── QUICK_REFERENCE.md         # This file
│
├── blog-engine/
│   ├── src/                   # Rust source
│   │   ├── main.rs
│   │   ├── handlers.rs
│   │   ├── models.rs
│   │   ├── db.rs
│   │   ├── auth.rs
│   │   └── utils.rs
│   ├── templates/             # HTML templates
│   ├── static/                # CSS/JS
│   ├── Cargo.toml
│   ├── README.md
│   └── demo.sh
│
├── chat-application/
│   ├── src/                   # Rust source
│   │   ├── main.rs
│   │   ├── server.rs
│   │   ├── models.rs
│   │   └── db.rs
│   ├── client/                # Web client
│   │   ├── index.html
│   │   ├── style.css
│   │   └── app.js
│   ├── Cargo.toml
│   └── README.md
│
└── package-manager/
    ├── src/                   # Rust source
    │   ├── main.rs
    │   ├── cli.rs
    │   ├── models.rs
    │   ├── resolver.rs
    │   ├── registry.rs
    │   ├── installer.rs
    │   └── lockfile.rs
    ├── registry-data/         # Sample packages
    ├── sample-project/
    ├── Cargo.toml
    ├── README.md
    └── demo.sh
```

---

## 🐛 Troubleshooting

### Blog Engine

**Port already in use:**
```bash
# Find process
lsof -i :8080
# Kill process
kill -9 <PID>
```

**Database locked:**
```bash
rm blog.db         # Delete and restart
```

### Chat Application

**Can't connect to WebSocket:**
- Ensure server is running: `cargo run`
- Check port 9001 is available
- Open browser console for errors

**Client not loading:**
- Open `client/index.html` directly in browser
- Check WebSocket URL in `app.js`

### Package Manager

**Package not found:**
- Check `registry-data/` directory
- Verify filename: `name-version.toml`
- Run from correct directory

**Circular dependency:**
- Review dependency chain
- Fix in Package.toml

---

## 📊 Performance Tips

### Release Builds
```bash
cargo build --release
# 10-100x faster than debug builds
```

### Parallel Builds
```bash
cargo build -j 8         # Use 8 cores
```

### Incremental Compilation
```bash
# Already enabled by default
export CARGO_INCREMENTAL=1
```

---

## 🎯 Quick Tests

### Test Blog Engine
```bash
cd blog-engine
cargo run &
sleep 5
curl http://127.0.0.1:8080/api/posts
kill %1
```

### Test Chat Application
```bash
cd chat-application
cargo run &
sleep 3
# Open client/index.html
kill %1
```

### Test Package Manager
```bash
cd package-manager
cargo build --release
./target/release/pkgmgr registry list
```

---

## 📚 Learn More

### Rust Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Crate Documentation
- [docs.rs](https://docs.rs/) - All crate documentation
- [crates.io](https://crates.io/) - Package registry

### Project READMEs
Each project has comprehensive documentation:
- `blog-engine/README.md` - 173 lines
- `chat-application/README.md` - 286 lines
- `package-manager/README.md` - 367 lines

---

## 💡 Quick Tips

1. **Always use `--release` for production**
2. **Read compiler errors carefully** - they're helpful!
3. **Use `cargo clippy`** for better code
4. **Run `cargo fmt`** before committing
5. **Check individual READMEs** for detailed info

---

## 🎉 Success!

You now have 3 complete, production-ready Rust projects:
- ✅ Web application with database
- ✅ Real-time WebSocket server
- ✅ CLI tool with graph algorithms

**Happy Coding! 🦀**
