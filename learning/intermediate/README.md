# Rust Intermediate Programs

This directory contains 6 intermediate-level Rust programs demonstrating various concepts and techniques.

## Programs

### 1. json_parser.rs
**JSON Parser with parsing, manipulation, and validation**
- Custom JSON parser implementation
- JSON value manipulation (get/set paths)
- Validation functions
- Pretty printing

**Compile & Run:**
```bash
rustc json_parser.rs && ./json_parser
```

### 2. web_scraper.rs
**HTTP client with HTML parsing and retry logic**
- HTTP client with all major methods (GET, POST, etc.)
- HTML tag extraction and parsing
- Exponential backoff retry mechanism
- Rate limiting demonstration

**Compile & Run:**
```bash
rustc web_scraper.rs && ./web_scraper
```

### 3. data_structures.rs
**Custom data structure implementations**
- Singly Linked List
- Binary Search Tree (BST)
- HashMap with separate chaining
- Full CRUD operations for each

**Compile & Run:**
```bash
rustc data_structures.rs && ./data_structures
```

### 4. sorting_algorithms.rs
**Sorting algorithms with benchmarking**
- Bubble Sort, Selection Sort, Insertion Sort
- Merge Sort, Quick Sort, Heap Sort
- Counting Sort (for integers)
- Comprehensive benchmarking suite
- Performance comparison across different input types

**Compile & Run:**
```bash
rustc -O sorting_algorithms.rs && ./sorting_algorithms
```
Note: Use `-O` flag for optimized compilation and accurate benchmarks.

### 5. file_processor.rs
**CSV processing with statistics and reports**
- CSV parser with custom delimiter support
- Statistical analysis (mean, median, std dev, min/max)
- Text and HTML report generation
- Column-based data extraction

**Compile & Run:**
```bash
rustc file_processor.rs && ./file_processor
```

### 6. api_client.rs
**REST API client with all HTTP methods**
- Support for GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS
- Authentication with Bearer tokens
- Custom headers and request builders
- Response parsing and error handling
- Timeout configuration

**Compile & Run:**
```bash
rustc api_client.rs && ./api_client
```

## Features

All programs include:
- ✅ Compile/run instructions at the top of each file
- ✅ Comprehensive comments and documentation
- ✅ Proper Rust idioms (Result types, Options, pattern matching)
- ✅ Error handling with custom error types
- ✅ Working demonstrations in main()
- ✅ Standalone and immediately runnable

## Notes

- Programs use mock/simulated implementations where external dependencies would normally be required (HTTP requests, etc.)
- For production use, consider using established crates like `reqwest`, `serde_json`, `scraper`, etc.
- All programs compile with standard `rustc` without additional dependencies
- Programs demonstrate intermediate concepts: custom types, trait implementations, generics, error handling, and more

## Quick Test All

```bash
cd /home/matthew/repos/Programming_Repos/rust-projects/learning/intermediate/

# Compile and run all programs
rustc json_parser.rs && ./json_parser
rustc web_scraper.rs && ./web_scraper
rustc data_structures.rs && ./data_structures
rustc -O sorting_algorithms.rs && ./sorting_algorithms
rustc file_processor.rs && ./file_processor
rustc api_client.rs && ./api_client
```
