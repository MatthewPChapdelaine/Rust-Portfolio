# Blog Engine

A full-featured blogging platform built with Rust, Actix-web, and SQLite.

## Features

- 🔐 **JWT Authentication** - Secure user registration and login
- 📝 **CRUD Operations** - Complete blog post management
- 📊 **SQLite Database** - Fast, embedded database with SQLx
- 🎨 **Markdown Support** - Write posts in Markdown with pulldown-cmark
- 💬 **Comments System** - Reader engagement with moderation
- 👑 **Admin Panel** - Manage posts and comments
- 🎭 **Tera Templates** - Server-side rendering
- 🚀 **RESTful API** - Clean API design
- 📱 **Responsive Design** - Works on all devices

## Tech Stack

- **Backend**: Actix-web 4.4
- **Database**: SQLite with SQLx 0.7
- **Auth**: JWT (jsonwebtoken)
- **Templates**: Tera 1.19
- **Markdown**: pulldown-cmark 0.9
- **Async Runtime**: Tokio 1.35

## Project Structure

```
blog-engine/
├── src/
│   ├── main.rs           # Application entry point
│   ├── handlers.rs       # HTTP request handlers
│   ├── models.rs         # Data models and DTOs
│   ├── db.rs            # Database operations
│   ├── auth.rs          # JWT authentication
│   └── utils.rs         # Utility functions
├── templates/
│   ├── layouts/         # Base templates
│   ├── blog/            # Blog page templates
│   └── admin/           # Admin panel templates
├── static/
│   ├── css/             # Stylesheets
│   └── js/              # JavaScript files
├── Cargo.toml           # Dependencies
└── README.md
```

## Getting Started

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (comes with Rust)

### Installation

1. Clone or navigate to the project:
```bash
cd blog-engine
```

2. Copy environment file:
```bash
cp .env.example .env
```

3. Build and run:
```bash
cargo build --release
cargo run
```

The server will start at `http://127.0.0.1:8080`

## Usage

### Creating an Admin User

First, register a user via the API:

```bash
curl -X POST http://127.0.0.1:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "email": "admin@example.com",
    "password": "password123"
  }'
```

Then manually update the user to admin in the database:

```bash
sqlite3 blog.db "UPDATE users SET is_admin = 1 WHERE username = 'admin';"
```

### Login

```bash
curl -X POST http://127.0.0.1:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "password123"
  }'
```

Save the returned token for authenticated requests.

### Creating a Post

```bash
curl -X POST http://127.0.0.1:8080/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{
    "title": "My First Post",
    "summary": "This is a summary of my first blog post",
    "content": "# Hello World\n\nThis is **markdown** content!",
    "published": true
  }'
```

### API Endpoints

#### Authentication
- `POST /api/register` - Register new user
- `POST /api/login` - Login and get JWT token

#### Posts
- `GET /api/posts` - List all posts (published only for non-admin)
- `POST /api/posts` - Create post (admin only)
- `GET /api/posts/{slug}` - Get single post
- `PUT /api/posts/{id}` - Update post (admin only)
- `DELETE /api/posts/{id}` - Delete post (admin only)

#### Comments
- `GET /api/posts/{slug}/comments` - Get post comments
- `POST /api/posts/{slug}/comments` - Create comment
- `POST /api/comments/{id}/approve` - Approve comment (admin only)
- `DELETE /api/comments/{id}` - Delete comment (admin only)

#### Web Pages
- `GET /` - Homepage with post list
- `GET /post/{slug}` - View single post
- `GET /admin` - Admin dashboard (requires auth)

## Configuration

Edit `.env` file:

```env
DATABASE_URL=sqlite://blog.db
JWT_SECRET=your-secret-key-change-in-production
HOST=127.0.0.1
PORT=8080
RUST_LOG=info
```

## Features in Detail

### Markdown Support

Posts support full Markdown syntax including:
- Headers, bold, italic, links
- Code blocks with syntax highlighting
- Tables and task lists
- Strikethrough and footnotes

### Security

- Passwords hashed with bcrypt
- JWT tokens with expiration
- SQL injection protection via SQLx
- CORS and compression middleware

### Error Handling

All operations use Rust's `Result` type for proper error handling. Database operations gracefully handle failures.

## Development

Run in development mode with auto-reload:

```bash
cargo watch -x run
```

Run tests:

```bash
cargo test
```

## Production Build

```bash
cargo build --release
./target/release/blog-engine
```

## License

MIT License - feel free to use this project for learning or production.

## Contributing

This is a learning project demonstrating Rust web development best practices:
- Actix-web for high-performance HTTP
- SQLx for type-safe SQL
- Proper error handling
- RESTful API design
- Secure authentication
- Clean code structure
