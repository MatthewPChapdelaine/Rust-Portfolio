/*!
 * Huffman Coding Compression Tool
 * 
 * A CLI tool implementing Huffman coding for data compression:
 * - Frequency analysis
 * - Huffman tree construction
 * - Encoding/decoding
 * - File compression and decompression
 * - Compression ratio statistics
 * 
 * # Compile and Run
 * ```bash
 * rustc compression_tool.rs -o compression_tool
 * 
 * # Compress
 * echo "Hello, World! This is a test." | ./compression_tool compress
 * 
 * # Decompress
 * ./compression_tool decompress <compressed_data>
 * 
 * # Interactive mode
 * ./compression_tool
 * ```
 */

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::io::{self, Read, Write};
use std::env;

// ============================================================================
// Huffman Tree Node
// ============================================================================

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    freq: usize,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new_leaf(ch: char, freq: usize) -> Self {
        Node {
            freq,
            ch: Some(ch),
            left: None,
            right: None,
        }
    }

    fn new_internal(freq: usize, left: Node, right: Node) -> Self {
        Node {
            freq,
            ch: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Huffman Encoder/Decoder
// ============================================================================

pub struct HuffmanCodec {
    root: Option<Node>,
    codes: HashMap<char, String>,
}

impl HuffmanCodec {
    /// Build Huffman tree from text
    pub fn from_text(text: &str) -> Result<Self, String> {
        if text.is_empty() {
            return Err("Empty input".to_string());
        }

        let frequencies = Self::calculate_frequencies(text);
        let root = Self::build_tree(frequencies)?;
        let codes = Self::generate_codes(&root);

        Ok(HuffmanCodec {
            root: Some(root),
            codes,
        })
    }

    /// Calculate character frequencies
    fn calculate_frequencies(text: &str) -> HashMap<char, usize> {
        let mut freq = HashMap::new();
        for ch in text.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }
        freq
    }

    /// Build Huffman tree using priority queue
    fn build_tree(frequencies: HashMap<char, usize>) -> Result<Node, String> {
        if frequencies.is_empty() {
            return Err("No characters to encode".to_string());
        }

        let mut heap = BinaryHeap::new();

        // Create leaf nodes
        for (ch, freq) in frequencies {
            heap.push(Node::new_leaf(ch, freq));
        }

        // Build tree bottom-up
        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            
            let parent = Node::new_internal(left.freq + right.freq, left, right);
            heap.push(parent);
        }

        heap.pop().ok_or_else(|| "Failed to build tree".to_string())
    }

    /// Generate Huffman codes by traversing tree
    fn generate_codes(root: &Node) -> HashMap<char, String> {
        let mut codes = HashMap::new();
        Self::generate_codes_helper(root, String::new(), &mut codes);
        codes
    }

    fn generate_codes_helper(node: &Node, code: String, codes: &mut HashMap<char, String>) {
        if node.is_leaf() {
            if let Some(ch) = node.ch {
                codes.insert(ch, if code.is_empty() { "0".to_string() } else { code });
            }
            return;
        }

        if let Some(ref left) = node.left {
            Self::generate_codes_helper(left, format!("{}0", code), codes);
        }

        if let Some(ref right) = node.right {
            Self::generate_codes_helper(right, format!("{}1", code), codes);
        }
    }

    /// Encode text to binary string
    pub fn encode(&self, text: &str) -> Result<String, String> {
        let mut encoded = String::new();
        
        for ch in text.chars() {
            match self.codes.get(&ch) {
                Some(code) => encoded.push_str(code),
                None => return Err(format!("Character '{}' not in codebook", ch)),
            }
        }

        Ok(encoded)
    }

    /// Decode binary string to text
    pub fn decode(&self, encoded: &str) -> Result<String, String> {
        if encoded.is_empty() {
            return Ok(String::new());
        }

        let root = self.root.as_ref().ok_or("No tree available")?;
        let mut decoded = String::new();
        let mut current = root;

        for bit in encoded.chars() {
            current = match bit {
                '0' => {
                    if let Some(ref left) = current.left {
                        left.as_ref()
                    } else {
                        return Err("Invalid encoding: unexpected '0'".to_string());
                    }
                }
                '1' => {
                    if let Some(ref right) = current.right {
                        right.as_ref()
                    } else {
                        return Err("Invalid encoding: unexpected '1'".to_string());
                    }
                }
                _ => return Err(format!("Invalid bit: {}", bit)),
            };

            if current.is_leaf() {
                if let Some(ch) = current.ch {
                    decoded.push(ch);
                    current = root;
                }
            }
        }

        Ok(decoded)
    }

    /// Get the code for a character
    pub fn get_code(&self, ch: char) -> Option<&String> {
        self.codes.get(&ch)
    }

    /// Get codebook
    pub fn codebook(&self) -> &HashMap<char, String> {
        &self.codes
    }

    /// Print tree structure (for debugging)
    pub fn print_tree(&self) {
        if let Some(ref root) = self.root {
            println!("Huffman Tree:");
            Self::print_tree_helper(root, 0);
        }
    }

    fn print_tree_helper(node: &Node, depth: usize) {
        let indent = "  ".repeat(depth);
        
        if node.is_leaf() {
            println!("{}Leaf: {:?} (freq: {})", indent, node.ch, node.freq);
        } else {
            println!("{}Internal (freq: {})", indent, node.freq);
            
            if let Some(ref left) = node.left {
                print!("{}├─0─ ", indent);
                Self::print_tree_helper(left, depth + 1);
            }
            
            if let Some(ref right) = node.right {
                print!("{}└─1─ ", indent);
                Self::print_tree_helper(right, depth + 1);
            }
        }
    }
}

// ============================================================================
// Compression Statistics
// ============================================================================

struct CompressionStats {
    original_size: usize,
    compressed_size: usize,
    compression_ratio: f64,
}

impl CompressionStats {
    fn calculate(original: &str, compressed: &str) -> Self {
        let original_size = original.len() * 8; // bits
        let compressed_size = compressed.len(); // already in bits
        let compression_ratio = 1.0 - (compressed_size as f64 / original_size as f64);

        CompressionStats {
            original_size,
            compressed_size,
            compression_ratio,
        }
    }

    fn print(&self) {
        println!("\n📊 Compression Statistics:");
        println!("  Original size:     {} bits ({} bytes)", self.original_size, self.original_size / 8);
        println!("  Compressed size:   {} bits ({:.1} bytes)", self.compressed_size, self.compressed_size as f64 / 8.0);
        println!("  Space saved:       {} bits ({:.1} bytes)", 
                 self.original_size - self.compressed_size,
                 (self.original_size - self.compressed_size) as f64 / 8.0);
        println!("  Compression ratio: {:.2}%", self.compression_ratio * 100.0);
    }
}

// ============================================================================
// CLI Interface
// ============================================================================

fn compress_text(text: &str) {
    println!("📝 Original text: {}", text);
    println!("   Length: {} characters\n", text.len());

    match HuffmanCodec::from_text(text) {
        Ok(codec) => {
            println!("🔧 Generated Huffman Codes:");
            let mut codes: Vec<_> = codec.codebook().iter().collect();
            codes.sort_by_key(|(_, code)| code.len());
            
            for (ch, code) in codes {
                let display_ch = if *ch == '\n' {
                    "\\n".to_string()
                } else if *ch == ' ' {
                    "' '".to_string()
                } else {
                    format!("'{}'", ch)
                };
                println!("  {} -> {}", display_ch, code);
            }

            match codec.encode(text) {
                Ok(encoded) => {
                    println!("\n🔐 Encoded (binary): {}", encoded);
                    
                    let stats = CompressionStats::calculate(text, &encoded);
                    stats.print();

                    // Verify by decoding
                    match codec.decode(&encoded) {
                        Ok(decoded) => {
                            if decoded == text {
                                println!("\n✅ Verification: Decode successful!");
                            } else {
                                println!("\n❌ Verification failed!");
                                println!("   Expected: {}", text);
                                println!("   Got:      {}", decoded);
                            }
                        }
                        Err(e) => println!("\n❌ Decode error: {}", e),
                    }
                }
                Err(e) => println!("❌ Encoding error: {}", e),
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn interactive_mode() {
    println!("🗜️  Huffman Coding Compression Tool");
    println!("====================================\n");
    println!("Commands:");
    println!("  1. compress <text>  - Compress text");
    println!("  2. demo            - Run demo");
    println!("  3. quit            - Exit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];

        match command {
            "compress" => {
                if parts.len() < 2 {
                    println!("Usage: compress <text>");
                    continue;
                }
                println!();
                compress_text(parts[1]);
                println!();
            }
            "demo" => {
                println!();
                run_demo();
                println!();
            }
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Unknown command. Try: compress, demo, or quit");
            }
        }
    }
}

fn run_demo() {
    let test_cases = vec![
        "AAAAAABBBBBBBBBBBCCCCCCCCCCCCDDDDDDDDDDDDD",
        "Hello, World!",
        "The quick brown fox jumps over the lazy dog",
        "aaaaaa",
        "abcdef",
    ];

    for (i, text) in test_cases.iter().enumerate() {
        println!("{:=^60}", format!(" Test Case {} ", i + 1));
        compress_text(text);
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        interactive_mode();
    } else if args.len() >= 2 {
        match args[1].as_str() {
            "compress" => {
                if args.len() >= 3 {
                    compress_text(&args[2..].join(" "));
                } else {
                    // Read from stdin
                    let mut input = String::new();
                    io::stdin().read_to_string(&mut input).unwrap();
                    compress_text(input.trim());
                }
            }
            "demo" => run_demo(),
            _ => {
                println!("Usage:");
                println!("  {} compress <text>", args[0]);
                println!("  {} demo", args[0]);
                println!("  {}  (interactive mode)", args[0]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_calculation() {
        let freq = HuffmanCodec::calculate_frequencies("aabbc");
        assert_eq!(freq.get(&'a'), Some(&2));
        assert_eq!(freq.get(&'b'), Some(&2));
        assert_eq!(freq.get(&'c'), Some(&1));
    }

    #[test]
    fn test_encode_decode() {
        let text = "hello world";
        let codec = HuffmanCodec::from_text(text).unwrap();
        let encoded = codec.encode(text).unwrap();
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(text, decoded);
    }

    #[test]
    fn test_single_character() {
        let codec = HuffmanCodec::from_text("aaaa").unwrap();
        let encoded = codec.encode("aaaa").unwrap();
        assert_eq!(encoded, "0000");
    }

    #[test]
    fn test_compression_ratio() {
        let text = "AAAAAABBBBBBBBBBBCCCCCCCCCCCCDDDDDDDDDDDDD";
        let codec = HuffmanCodec::from_text(text).unwrap();
        let encoded = codec.encode(text).unwrap();
        
        let original_bits = text.len() * 8;
        let compressed_bits = encoded.len();
        
        assert!(compressed_bits < original_bits);
    }
}
