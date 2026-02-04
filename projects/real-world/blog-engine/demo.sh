#!/bin/bash
# Demo script for blog engine

cd "$(dirname "$0")"

echo "=== Blog Engine Demo ==="
echo ""

# Copy .env file
cp .env.example .env

echo "Starting blog server in background..."
cargo run > server.log 2>&1 &
SERVER_PID=$!

# Wait for server to start
echo "Waiting for server to initialize..."
sleep 8

echo ""
echo "1. Registering admin user:"
curl -s -X POST http://127.0.0.1:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@example.com","password":"admin123"}' | jq .

echo ""
echo "2. Logging in:"
TOKEN=$(curl -s -X POST http://127.0.0.1:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}' | jq -r .token)

echo "Got token: ${TOKEN:0:50}..."

# Make user admin manually
sqlite3 blog.db "UPDATE users SET is_admin = 1 WHERE username = 'admin';"

echo ""
echo "3. Creating a blog post:"
curl -s -X POST http://127.0.0.1:8080/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "title": "Welcome to Rust Blog Engine",
    "summary": "This is the first post on our new Rust-powered blog platform!",
    "content": "# Hello World\n\nThis is a **test post** written in *Markdown*.\n\n## Features\n\n- JWT Authentication\n- Markdown Support\n- SQLite Database\n- RESTful API\n\n```rust\nfn main() {\n    println!(\"Hello from Rust!\");\n}\n```",
    "published": true
  }' | jq .

echo ""
echo "4. Getting all posts:"
curl -s http://127.0.0.1:8080/api/posts | jq .

echo ""
echo ""
echo "Server running at: http://127.0.0.1:8080"
echo "View in browser or stop server with: kill $SERVER_PID"
echo ""
echo "To view logs: tail -f server.log"
