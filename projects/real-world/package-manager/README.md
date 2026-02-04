# Package Manager (pkgmgr)

A Cargo-like package manager built in Rust, featuring dependency resolution, version management, lock file generation, and dependency graph visualization.

## Features

- 📦 **Dependency Resolution** - Semantic versioning with automatic resolution
- 🔒 **Lock File Generation** - Cargo.lock-style deterministic builds
- 🌳 **Dependency Tree Visualization** - View dependency graph with petgraph
- 📚 **Local Registry** - Simulated crates.io with TOML manifests
- 🔍 **Package Search** - Search and discover packages
- ⚡ **CLI Interface** - Clean command-line interface with clap
- ✅ **Cycle Detection** - Prevents circular dependencies
- 🎨 **Colored Output** - Beautiful terminal interface

## Tech Stack

- **CLI**: clap 4.4 with derive macros
- **Parsing**: toml, serde, serde_json
- **Versioning**: semver 1.0
- **Graphs**: petgraph 0.6
- **Errors**: anyhow, thiserror
- **UI**: colored 2.1

## Project Structure

```
package-manager/
├── src/
│   ├── main.rs           # Entry point and CLI commands
│   ├── cli.rs            # Command-line interface definitions
│   ├── models.rs         # Data structures
│   ├── resolver.rs       # Dependency resolution logic
│   ├── registry.rs       # Package registry management
│   ├── installer.rs      # Package installation
│   └── lockfile.rs       # Lock file generation
├── registry-data/        # Simulated package registry
│   ├── serde-1.0.195.toml
│   ├── tokio-1.35.1.toml
│   └── ...
├── sample-project/       # Example project
│   └── Package.toml
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)

### Building

```bash
cd package-manager
cargo build --release
```

The binary will be at `target/release/pkgmgr`

### Installation (Optional)

```bash
cargo install --path .
```

This installs `pkgmgr` to your Cargo bin directory.

## Usage

### Initialize a New Package

```bash
pkgmgr init my-project
```

Creates a new `Package.toml` manifest:

```toml
[package]
name = "my-project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A new package"

[dependencies]
```

### Install Dependencies

```bash
pkgmgr install
```

This will:
1. Read `Package.toml`
2. Resolve all dependencies with semantic versioning
3. Check for circular dependencies
4. Install packages to `pkg_modules/`
5. Generate `Package.lock` file

### Update Dependencies

```bash
pkgmgr update
```

Removes the lock file and re-resolves all dependencies to get the latest compatible versions.

### View Dependency Tree

```bash
pkgmgr tree
```

Displays a visual dependency graph:

```
🌳 Dependency tree:

├─ actix-web v4.4.0
  ├─ tokio v1.35.1
  ├─ serde v1.0.195
├─ serde v1.0.195
├─ clap v4.4.18

✓ 4 total packages
```

### Registry Commands

#### List All Packages

```bash
pkgmgr registry list
```

Output:
```
📚 Available packages:

  • actix-web
    versions: 4.4.0
  • serde
    versions: 1.0.195
  • tokio
    versions: 1.35.1
  ...
```

#### Search Packages

```bash
pkgmgr registry search web
```

#### Show Package Info

```bash
pkgmgr registry info actix-web
```

Output:
```
ℹ️ Package info: actix-web

  Name:        actix-web
  Version:     4.4.0
  Authors:     Actix Team
  Description: Actix Web is a powerful, pragmatic, and fast web framework

  Dependencies:
    tokio ^1.0
    serde ^1.0
```

## Package.toml Format

The manifest file uses TOML format similar to Cargo.toml:

```toml
[package]
name = "my-app"
version = "0.1.0"
authors = ["John Doe <john@example.com>"]
description = "My awesome application"

[dependencies]
serde = "^1.0"        # Caret: >=1.0.0, <2.0.0
tokio = "~1.35"       # Tilde: >=1.35.0, <1.36.0
actix-web = "4.4.0"   # Exact version
reqwest = "*"         # Any version (not recommended)
```

## Version Requirements

Follows semantic versioning (semver):

- `^1.2.3` - Caret: `>=1.2.3 <2.0.0`
- `~1.2.3` - Tilde: `>=1.2.3 <1.3.0`
- `1.2.3` - Exact: `=1.2.3`
- `>=1.2.3` - Greater than or equal
- `*` - Any version

## Package.lock Format

Generated lock file ensures reproducible builds:

```toml
version = "1.0"

[[packages]]
name = "actix-web"
version = "4.4.0"
dependencies = ["tokio", "serde"]
checksum = "a3d8b9..."

[[packages]]
name = "serde"
version = "1.0.195"
dependencies = []
checksum = "7f2c1a..."
```

## Architecture

### Dependency Resolution Algorithm

1. **Parse Manifest**: Read `Package.toml` and extract dependencies
2. **Queue Dependencies**: Add all direct dependencies to resolution queue
3. **Resolve Versions**: For each dependency:
   - Query registry for matching version
   - Add transitive dependencies to queue
   - Track visited packages to avoid duplicates
4. **Check Cycles**: Build dependency graph and detect cycles
5. **Generate Output**: Create resolved package list

### Registry Structure

The registry is a directory of TOML files:

```
registry-data/
├── package-name-version.toml
├── serde-1.0.195.toml
└── tokio-1.35.1.toml
```

Each file contains package metadata and dependencies.

### Dependency Graph

Uses `petgraph` to build a directed graph:
- Nodes: Packages with versions
- Edges: Dependency relationships
- Algorithms: BFS for traversal, cycle detection

## Examples

### Try the Sample Project

```bash
cd sample-project
../target/release/pkgmgr install
../target/release/pkgmgr tree
```

### Create Your Own Registry Package

```bash
cat > registry-data/mylib-1.0.0.toml << EOF
name = "mylib"
version = "1.0.0"
authors = ["Me <me@example.com>"]
description = "My library"

[dependencies]
serde = "^1.0"
EOF
```

### Add to Your Project

```toml
[dependencies]
mylib = "^1.0"
```

## Command Reference

| Command | Description |
|---------|-------------|
| `pkgmgr init <name>` | Initialize new package |
| `pkgmgr install` | Install dependencies |
| `pkgmgr install <pkg>` | Install specific package |
| `pkgmgr update` | Update all dependencies |
| `pkgmgr tree` | Show dependency tree |
| `pkgmgr registry list` | List all packages |
| `pkgmgr registry search <query>` | Search packages |
| `pkgmgr registry info <package>` | Show package details |

## Error Handling

Robust error handling with `anyhow`:
- Package not found errors
- Version resolution failures
- Circular dependency detection
- File I/O errors
- Invalid TOML parsing

All errors include helpful context messages.

## Features in Detail

### Semantic Versioning

Full semver 1.0 implementation:
- Version parsing and comparison
- Version requirement matching
- Automatic resolution to latest compatible version

### Cycle Detection

Prevents infinite loops in dependency resolution:
```rust
A -> B -> C -> A  // Error: circular dependency detected
```

### Checksum Generation

Each package in lock file has SHA-256 checksum for integrity verification.

### Graph Visualization

Pretty-printed dependency tree:
- Color-coded output
- Hierarchical structure
- Duplicate detection (marked with *)

## Performance

- **Lock-free**: No unnecessary locking
- **Efficient Graph**: O(V + E) traversal
- **Smart Caching**: Registry loaded once
- **Parallel Ready**: Architecture supports parallel resolution

## Limitations & Future Improvements

Current limitations:
- Simulated registry (not network-based)
- No actual package downloading
- Simple conflict resolution
- No dev/build dependencies

Future enhancements:
- HTTP registry client
- Real package downloads
- Advanced conflict resolution
- Workspace support
- Build scripts
- Feature flags

## Testing

Add packages to `registry-data/`:

```bash
echo 'name = "testpkg"
version = "1.0.0"
authors = ["Test"]

[dependencies]' > registry-data/testpkg-1.0.0.toml
```

Create test project:

```bash
pkgmgr init test-project
cd test-project
# Edit Package.toml to add testpkg
pkgmgr install
```

## Troubleshooting

**"Package not found" error:**
- Check package exists in `registry-data/`
- Verify filename format: `name-version.toml`
- Ensure version matches requirement

**"Circular dependency" error:**
- Review dependency chain in error message
- Break the cycle by removing/changing a dependency

**Lock file mismatch:**
- Run `pkgmgr update` to regenerate
- Delete `Package.lock` and reinstall

## Development

### Project Layout

```
src/
├── main.rs       - CLI commands and main logic
├── cli.rs        - Clap command definitions
├── models.rs     - Manifest, Package, Registry types
├── resolver.rs   - Dependency resolution algorithm
├── registry.rs   - Package registry operations
├── installer.rs  - Package installation logic
└── lockfile.rs   - Lock file generation and parsing
```

### Adding New Commands

1. Add variant to `Commands` enum in `cli.rs`
2. Implement handler in `main.rs`
3. Use existing modules for logic

## License

MIT License - Free to use for learning and projects.

## Acknowledgments

Inspired by:
- **Cargo** - Rust's package manager
- **npm** - Node.js package manager
- **pip** - Python package installer

This project demonstrates:
- CLI application development with clap
- Graph algorithms with petgraph
- Semantic versioning
- File I/O and parsing
- Error handling patterns
- Rust best practices
