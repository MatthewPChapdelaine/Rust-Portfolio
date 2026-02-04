# Quick Start Guide

## Compile All Programs

```bash
cd /home/matthew/repos/Programming_Repos/rust-projects/learning/advanced/

# Compile all (takes ~2 minutes)
for file in *.rs; do
    echo "Compiling ${file}..."
    rustc --edition 2021 "$file" -o "${file%.rs}"
done

# Or with optimizations
for file in *.rs; do
    echo "Compiling ${file} with optimizations..."
    rustc -O --edition 2021 "$file" -o "${file%.rs}"
done
```

## Quick Test Each Program

### 1. Design Patterns
```bash
./design_patterns
# Shows all 6 design patterns in action
```

### 2. Graph Algorithms
```bash
./graph_algorithms
# Demonstrates Dijkstra, BFS, DFS, topological sort, cycle detection
```

### 3. Expression Parser
```bash
./lexer_parser "2 + 3 * 4"
./lexer_parser "(10 + 5) / 3"
./lexer_parser test  # Run test suite
```

### 4. Compression Tool
```bash
echo "Hello World!" | ./compression_tool compress
./compression_tool demo
```

### 5. Database ORM
```bash
./database_orm
# Creates users and posts, demonstrates query builder
```

### 6. Memory Pool
```bash
./memory_pool
# Shows object pooling with benchmarks
```

### 7. Multi-threaded Server
```bash
# Terminal 1
./multi_threaded_server

# Terminal 2
telnet localhost 7878
# Type: ECHO Hello World
# Type: STATS
# Type: QUIT
```

### 8. Web Framework
```bash
# Terminal 1
./web_framework

# Terminal 2
curl http://localhost:8080/
curl http://localhost:8080/hello/Rust
curl http://localhost:8080/json
curl "http://localhost:8080/echo?msg=Testing"
```

## File Structure

```
advanced/
├── README.md                      # Comprehensive documentation
├── QUICK_START.md                 # This file
├── multi_threaded_server.rs       # TCP server with thread pool
├── design_patterns.rs             # 6 design patterns
├── web_framework.rs               # Mini web framework
├── database_orm.rs                # Simple ORM
├── graph_algorithms.rs            # Graph algorithms
├── compression_tool.rs            # Huffman coding
├── memory_pool.rs                 # Object pool
└── lexer_parser.rs                # Expression parser
```

## Lines of Code

```
multi_threaded_server.rs:    ~280 lines
design_patterns.rs:          ~470 lines
web_framework.rs:            ~480 lines
database_orm.rs:             ~520 lines
graph_algorithms.rs:         ~580 lines
compression_tool.rs:         ~480 lines
memory_pool.rs:              ~470 lines
lexer_parser.rs:             ~580 lines
─────────────────────────────────────
Total:                       ~3,860 lines
```

## Key Features

✅ All programs compile without errors
✅ Production-quality error handling
✅ Comprehensive documentation
✅ Unit tests included
✅ Interactive and CLI modes
✅ Idiomatic Rust patterns
✅ Zero unsafe code
✅ Thread-safe implementations

## Performance Notes

- Compile with `-O` flag for benchmarks
- Multi-threaded server handles multiple clients concurrently
- Memory pool shows significant reuse rates (>70%)
- Graph algorithms use efficient data structures
- Compression achieves good ratios on text

## Learning Path

**Beginner → Intermediate:**
1. compression_tool.rs (data structures)
2. lexer_parser.rs (parsing)
3. design_patterns.rs (patterns)

**Intermediate → Advanced:**
4. graph_algorithms.rs (algorithms)
5. memory_pool.rs (memory management)
6. database_orm.rs (abstraction)

**Advanced:**
7. multi_threaded_server.rs (concurrency)
8. web_framework.rs (systems programming)

---

**Ready to explore advanced Rust! 🦀**
