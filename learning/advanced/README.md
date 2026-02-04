# Advanced Rust Programming Projects

A collection of 8 production-quality Rust programs demonstrating advanced concepts, patterns, and algorithms. Each program includes comprehensive documentation, error handling, tests, and practical examples.

## 📚 Programs Overview

### 1. Multi-threaded TCP Server (`multi_threaded_server.rs`)
A concurrent TCP server with thread pool architecture for handling multiple clients simultaneously.

**Features:**
- Custom thread pool implementation
- Connection statistics tracking
- Command protocol (ECHO, STATS, SLEEP, QUIT)
- Graceful shutdown handling
- Thread-safe shared state

**Compile & Run:**
```bash
rustc multi_threaded_server.rs -o multi_threaded_server
./multi_threaded_server
```

**Test:**
```bash
# Terminal 1
./multi_threaded_server

# Terminal 2+
telnet localhost 7878
# or
echo "ECHO Hello World" | nc localhost 7878
```

**Key Concepts:** Multi-threading, synchronization (Arc, Mutex), TCP networking, RAII pattern

---

### 2. Design Patterns (`design_patterns.rs`)
Implementation of 6 classic design patterns adapted for idiomatic Rust.

**Patterns Implemented:**
- **Singleton**: Thread-safe singleton using `Once`
- **Factory**: Dynamic object creation with trait objects
- **Observer**: Publish-subscribe pattern with Rc/RefCell
- **Strategy**: Runtime polymorphism for algorithms
- **Decorator**: Wrapper pattern for extending behavior
- **Builder**: Fluent API for object construction

**Compile & Run:**
```bash
rustc design_patterns.rs -o design_patterns
./design_patterns
```

**Key Concepts:** Trait objects, interior mutability, type-state pattern, zero-cost abstractions

---

### 3. Mini Web Framework (`web_framework.rs`)
A lightweight web framework built from scratch with HTTP server, routing, and middleware.

**Features:**
- HTTP request/response parsing
- Pattern-based routing with path parameters
- Middleware chain support
- Query parameter parsing
- JSON response helpers
- Multi-threaded request handling

**Compile & Run:**
```bash
rustc web_framework.rs -o web_framework
./web_framework
```

**Test:**
```bash
curl http://localhost:8080/
curl http://localhost:8080/hello/YourName
curl http://localhost:8080/json
curl http://localhost:8080/echo?msg=Hello
curl -X POST http://localhost:8080/data -d "test data"
```

**Key Concepts:** HTTP protocol, functional composition, closure-based handlers, Arc for shared state

---

### 4. Database ORM (`database_orm.rs`)
A simple Object-Relational Mapping system with type-safe query building.

**Features:**
- Generic model trait for database entities
- CRUD operations (Create, Read, Update, Delete)
- Query builder with method chaining
- Type-safe value representation
- Repository pattern
- Migration support

**Compile & Run:**
```bash
rustc database_orm.rs -o database_orm
./database_orm
```

**Note:** This is a demonstration using in-memory storage. For production use with SQLite, add the `rusqlite` crate to Cargo.toml.

**Key Concepts:** Trait-based abstraction, builder pattern, type safety, generics

---

### 5. Graph Algorithms (`graph_algorithms.rs`)
Comprehensive implementation of essential graph algorithms.

**Algorithms Included:**
- **Dijkstra's Algorithm**: Shortest path with weighted edges
- **BFS (Breadth-First Search)**: Level-order traversal
- **DFS (Depth-First Search)**: Recursive and iterative
- **Topological Sort**: Two implementations (DFS & Kahn's)
- **Cycle Detection**: For directed and undirected graphs
- **Connected Components**: Finding graph partitions

**Compile & Run:**
```bash
rustc graph_algorithms.rs -o graph_algorithms
./graph_algorithms
```

**Key Concepts:** Graph data structures, priority queues (BinaryHeap), recursion, algorithmic complexity

---

### 6. Huffman Compression Tool (`compression_tool.rs`)
A CLI tool implementing Huffman coding for lossless data compression.

**Features:**
- Frequency analysis
- Huffman tree construction using priority queue
- Binary encoding/decoding
- Compression statistics
- Interactive and batch modes
- Verification of encode/decode roundtrip

**Compile & Run:**
```bash
rustc compression_tool.rs -o compression_tool

# Interactive mode
./compression_tool

# Compress text
echo "Hello, World!" | ./compression_tool compress

# Run demo
./compression_tool demo
```

**Key Concepts:** Binary trees, priority queues, greedy algorithms, bit manipulation

---

### 7. Memory Pool (`memory_pool.rs`)
Thread-safe object pool implementation for efficient memory reuse.

**Features:**
- Generic pool for any type
- Automatic return-to-pool on drop (RAII)
- Thread-safe with Arc/Mutex
- Statistics tracking (created, reused, peak size)
- Performance benchmarks
- Buffer pool specialization

**Compile & Run:**
```bash
# Without optimizations
rustc memory_pool.rs -o memory_pool

# With optimizations for better benchmark results
rustc -O memory_pool.rs -o memory_pool

./memory_pool
```

**Key Concepts:** Smart pointers, Drop trait, object pooling, performance optimization, benchmarking

---

### 8. Expression Lexer & Parser (`lexer_parser.rs`)
Complete lexer and parser for arithmetic expressions with evaluation.

**Features:**
- Tokenization (lexer) with position tracking
- Recursive descent parser
- Abstract Syntax Tree (AST) construction
- Operator precedence (following PEMDAS)
- Right-associative exponentiation
- Unary operators support
- Expression evaluation with error handling

**Supported Operations:**
- Addition (`+`), Subtraction (`-`)
- Multiplication (`*`), Division (`/`)
- Exponentiation (`^`)
- Parentheses `( )`
- Unary minus (`-x`)

**Compile & Run:**
```bash
rustc lexer_parser.rs -o lexer_parser

# Interactive mode
./lexer_parser

# Evaluate expression
./lexer_parser "2 + 3 * 4"
./lexer_parser "(5 + 3) * 2 - 4"
./lexer_parser "2 ^ 3 ^ 2"

# Run test suite
./lexer_parser test
```

**Key Concepts:** Lexical analysis, syntax parsing, AST, recursive descent, operator precedence

---

## 🔧 Compilation Tips

### Standard Compilation
```bash
rustc --edition 2021 <filename>.rs -o <output>
```

### With Optimizations
```bash
rustc -O --edition 2021 <filename>.rs -o <output>
```

### Compile All
```bash
for file in *.rs; do
    rustc --edition 2021 "$file" -o "${file%.rs}"
done
```

### Run Tests (if available)
Most programs include unit tests. To run them in a Cargo project:
```bash
# Create Cargo.toml and move source files
cargo test
```

---

## 📖 Learning Objectives

Each program demonstrates multiple Rust concepts:

### Ownership & Borrowing
- Smart pointers: `Box`, `Rc`, `Arc`
- Interior mutability: `RefCell`, `Mutex`
- Lifetime management
- Move semantics

### Concurrency
- Thread creation and management
- Synchronization primitives
- Message passing with channels
- Thread-safe shared state

### Error Handling
- `Result<T, E>` for recoverable errors
- Custom error types
- Error propagation with `?`
- Comprehensive error messages

### Type System
- Traits and trait objects
- Generics with bounds
- Associated types
- Type inference

### Patterns & Idioms
- Builder pattern
- RAII (Resource Acquisition Is Initialization)
- Newtype pattern
- Type-state pattern
- Zero-cost abstractions

### Data Structures
- Trees (binary, Huffman)
- Graphs (adjacency list)
- Priority queues
- Hash maps and sets

### Algorithms
- Searching (BFS, DFS)
- Shortest path (Dijkstra)
- Sorting (topological)
- Compression (Huffman)
- Parsing (recursive descent)

---

## 🎯 Usage Scenarios

- **Multi-threaded Server**: Learn concurrent programming, build network services
- **Design Patterns**: Understand Rust-idiomatic pattern implementations
- **Web Framework**: Build HTTP services, understand web protocols
- **Database ORM**: Create type-safe database abstractions
- **Graph Algorithms**: Solve pathfinding, dependency resolution problems
- **Compression Tool**: Understand information theory, build compression utilities
- **Memory Pool**: Optimize performance-critical applications
- **Lexer/Parser**: Build DSLs, interpreters, or configuration parsers

---

## 🧪 Testing

Each program includes:
- Unit tests (run with `cargo test`)
- Integration examples in `main()`
- Error handling demonstrations
- Edge case coverage

---

## 📝 Best Practices Demonstrated

1. **Documentation**: Comprehensive doc comments for all public APIs
2. **Error Handling**: Proper use of `Result` and custom error types
3. **Type Safety**: Leveraging Rust's type system for correctness
4. **Memory Safety**: No unsafe code, proper lifetime management
5. **Idiomatic Code**: Following Rust conventions and style guidelines
6. **Performance**: Efficient algorithms and data structure choices
7. **Testing**: Unit tests for critical functionality
8. **Modularity**: Clean separation of concerns

---

## 🚀 Next Steps

To extend these programs:

1. **Multi-threaded Server**: Add SSL/TLS, HTTP protocol, authentication
2. **Design Patterns**: Add Command, State, Visitor patterns
3. **Web Framework**: Add sessions, cookies, template rendering
4. **Database ORM**: Integrate real SQLite, add transactions, migrations
5. **Graph Algorithms**: Add A*, Bellman-Ford, minimum spanning tree
6. **Compression Tool**: Add LZW, file I/O, parallel compression
7. **Memory Pool**: Add size classes, better statistics, async support
8. **Lexer/Parser**: Add variables, functions, more operators

---

## 📚 References & Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Rustonomicon (Advanced Topics)](https://doc.rust-lang.org/nomicon/)

---

## ⚖️ License

These programs are created for educational purposes. Feel free to use, modify, and learn from them.

---

## 🤝 Contributing

These are learning examples. To improve them:
1. Add more test cases
2. Improve error messages
3. Add more features
4. Optimize performance
5. Enhance documentation

---

**Happy Rust Learning! 🦀**
