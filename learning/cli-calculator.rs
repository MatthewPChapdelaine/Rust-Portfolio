/*
 * CLI Calculator - Perform basic arithmetic operations
 * Compile: rustc cli-calculator.rs
 * Run: ./cli-calculator
 */

use std::io;

fn main() {
    println!("=== CLI Calculator ===");
    println!("Operations: +, -, *, /");
    
    let mut num1_str = String::new();
    print!("Enter first number: ");
    io::stdin().read_line(&mut num1_str).expect("Failed to read");
    let num1: f64 = match num1_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: Invalid number input");
            return;
        }
    };
    
    let mut operator = String::new();
    println!("Enter operator (+, -, *, /): ");
    io::stdin().read_line(&mut operator).expect("Failed to read");
    let operator = operator.trim();
    
    let mut num2_str = String::new();
    println!("Enter second number: ");
    io::stdin().read_line(&mut num2_str).expect("Failed to read");
    let num2: f64 = match num2_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: Invalid number input");
            return;
        }
    };
    
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Cannot divide by zero");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Error: Invalid operator");
            return;
        }
    };
    
    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}
