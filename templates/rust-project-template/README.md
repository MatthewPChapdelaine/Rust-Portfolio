# Rust Project Template

A basic Rust project structure with Cargo.

## Structure

```
rust-project-template/
├── src/
│   └── main.rs          # Main application
├── Cargo.toml           # Project configuration
└── README.md           # This file
```

## Setup

```bash
# Build the project
cargo build
```

## Usage

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Development

1. Make your changes in `src/main.rs` or create new modules in `src/`
2. Write tests inline with `#[cfg(test)]` or in `tests/` directory
3. Run tests with `cargo test`
4. Build release version with `cargo build --release`
