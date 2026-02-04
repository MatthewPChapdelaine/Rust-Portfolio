/*
 * File Reader - Read and display file contents
 * Compile: rustc file-reader.rs
 * Run: ./file-reader <filename>
 */

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    
    let filename = &args[1];
    
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("=== Contents of {} ===", filename);
            print!("{}", contents);
            println!("\n=== End of file ({} characters) ===", contents.len());
        }
        Err(e) => {
            println!("Error reading file '{}': {}", filename, e);
        }
    }
}
