/*!
 * Expression Lexer and Parser
 * 
 * A complete implementation of lexer and parser for arithmetic expressions:
 * - Tokenization (lexer)
 * - Abstract Syntax Tree (AST)
 * - Recursive descent parser
 * - Expression evaluation
 * - Operator precedence handling
 * - Error reporting with position tracking
 * 
 * # Compile and Run
 * ```bash
 * rustc lexer_parser.rs -o lexer_parser
 * ./lexer_parser
 * 
 * # Interactive mode:
 * ./lexer_parser "3 + 4 * 2"
 * ./lexer_parser "(5 + 3) * 2 - 4"
 * ```
 * 
 * # Supported Operations
 * - Addition: +
 * - Subtraction: -
 * - Multiplication: *
 * - Division: /
 * - Exponentiation: ^
 * - Parentheses: ( )
 * - Unary minus: -x
 */

use std::fmt;
use std::env;
use std::io::{self, Write};

// ============================================================================
// Token Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParen,
    RightParen,
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Caret => write!(f, "^"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

// ============================================================================
// Lexer
// ============================================================================

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse::<f64>()
            .map_err(|_| format!("Invalid number: {}", num_str))
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.current_char {
            None => Ok(Token::Eof),
            Some(ch) => {
                if ch.is_ascii_digit() {
                    return Ok(Token::Number(self.read_number()?));
                }

                let token = match ch {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Star,
                    '/' => Token::Slash,
                    '^' => Token::Caret,
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    _ => return Err(format!("Unexpected character: '{}'", ch)),
                };

                self.advance();
                Ok(token)
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }
}

// ============================================================================
// Abstract Syntax Tree (AST)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Number(f64),
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<AstNode>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Negate,
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstNode::Number(n) => write!(f, "{}", n),
            AstNode::BinaryOp { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            AstNode::UnaryOp { op, operand } => {
                write!(f, "({}{})", op, operand)
            }
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Power => write!(f, "^"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
        }
    }
}

// ============================================================================
// Parser (Recursive Descent)
// ============================================================================

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current_token()))
        }
    }

    /// Parse entry point
    /// Grammar: expression -> term ((PLUS | MINUS) term)*
    pub fn parse(&mut self) -> Result<AstNode, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<AstNode, String> {
        let mut node = self.parse_term()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_term()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    /// Grammar: term -> factor ((STAR | SLASH) factor)*
    fn parse_term(&mut self) -> Result<AstNode, String> {
        let mut node = self.parse_power()?;

        while matches!(self.current_token(), Token::Star | Token::Slash) {
            let op = match self.current_token() {
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_power()?;
            node = AstNode::BinaryOp {
                op,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    /// Grammar: power -> unary (CARET unary)*
    /// Right-associative: 2^3^4 = 2^(3^4)
    fn parse_power(&mut self) -> Result<AstNode, String> {
        let mut node = self.parse_unary()?;

        if matches!(self.current_token(), Token::Caret) {
            self.advance();
            let right = self.parse_power()?; // Right-associative
            node = AstNode::BinaryOp {
                op: BinaryOperator::Power,
                left: Box::new(node),
                right: Box::new(right),
            };
        }

        Ok(node)
    }

    /// Grammar: unary -> (PLUS | MINUS) unary | primary
    fn parse_unary(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(AstNode::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(operand),
                })
            }
            Token::Plus => {
                self.advance();
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    /// Grammar: primary -> NUMBER | LPAREN expression RPAREN
    fn parse_primary(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Ok(AstNode::Number(num))
            }
            Token::LeftParen => {
                self.advance();
                let node = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(node)
            }
            token => Err(format!("Unexpected token: {:?}", token)),
        }
    }
}

// ============================================================================
// Evaluator
// ============================================================================

pub fn evaluate(node: &AstNode) -> Result<f64, String> {
    match node {
        AstNode::Number(n) => Ok(*n),
        AstNode::BinaryOp { op, left, right } => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;

            match op {
                BinaryOperator::Add => Ok(left_val + right_val),
                BinaryOperator::Subtract => Ok(left_val - right_val),
                BinaryOperator::Multiply => Ok(left_val * right_val),
                BinaryOperator::Divide => {
                    if right_val == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                BinaryOperator::Power => Ok(left_val.powf(right_val)),
            }
        }
        AstNode::UnaryOp { op, operand } => {
            let val = evaluate(operand)?;
            match op {
                UnaryOperator::Negate => Ok(-val),
            }
        }
    }
}

// ============================================================================
// Pretty Printer
// ============================================================================

pub fn print_ast(node: &AstNode, indent: usize) {
    let prefix = "  ".repeat(indent);
    
    match node {
        AstNode::Number(n) => {
            println!("{}Number({})", prefix, n);
        }
        AstNode::BinaryOp { op, left, right } => {
            println!("{}BinaryOp({:?})", prefix, op);
            print_ast(left, indent + 1);
            print_ast(right, indent + 1);
        }
        AstNode::UnaryOp { op, operand } => {
            println!("{}UnaryOp({:?})", prefix, op);
            print_ast(operand, indent + 1);
        }
    }
}

// ============================================================================
// CLI Interface
// ============================================================================

fn process_expression(expr: &str) {
    println!("\n📝 Expression: {}", expr);
    
    // Lexing
    let mut lexer = Lexer::new(expr);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            println!("❌ Lexer error: {}", e);
            return;
        }
    };
    
    print!("🔤 Tokens: ");
    for (i, token) in tokens.iter().enumerate() {
        if i > 0 && token != &Token::Eof {
            print!(", ");
        }
        if token != &Token::Eof {
            print!("{:?}", token);
        }
    }
    println!();
    
    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            println!("❌ Parser error: {}", e);
            return;
        }
    };
    
    println!("\n🌳 Abstract Syntax Tree:");
    print_ast(&ast, 0);
    
    println!("\n📐 S-Expression: {}", ast);
    
    // Evaluation
    match evaluate(&ast) {
        Ok(result) => {
            println!("\n✅ Result: {}", result);
        }
        Err(e) => {
            println!("\n❌ Evaluation error: {}", e);
        }
    }
}

fn interactive_mode() {
    println!("🔢 Expression Parser & Evaluator");
    println!("=================================\n");
    println!("Enter arithmetic expressions to evaluate.");
    println!("Supported operators: +, -, *, /, ^ (power), ( )");
    println!("Type 'quit' or 'exit' to quit.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "exit" {
            println!("Goodbye!");
            break;
        }

        process_expression(input);
    }
}

fn run_tests() {
    println!("🧪 Running Test Cases\n");
    println!("{:=^60}", "");
    
    let test_cases = vec![
        "3 + 4",
        "10 - 5",
        "2 * 3",
        "15 / 3",
        "2 ^ 3",
        "2 + 3 * 4",
        "(2 + 3) * 4",
        "10 - 2 - 3",
        "-5 + 3",
        "2 * -3",
        "(5 + 3) * (2 - 1)",
        "2 ^ 3 ^ 2",
        "100 / 10 / 2",
        "3.14 * 2",
        "-(4 + 5)",
    ];

    for expr in test_cases {
        process_expression(expr);
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        interactive_mode();
    } else if args.len() == 2 {
        if args[1] == "test" {
            run_tests();
        } else {
            process_expression(&args[1]);
        }
    } else {
        println!("Usage:");
        println!("  {}              # Interactive mode", args[0]);
        println!("  {} <expression>  # Evaluate expression", args[0]);
        println!("  {} test         # Run test cases", args[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("3 + 4");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(3.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(4.0));
    }

    #[test]
    fn test_parser_simple() {
        let mut lexer = Lexer::new("3 + 4");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluate(&ast).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_precedence() {
        let mut lexer = Lexer::new("2 + 3 * 4");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluate(&ast).unwrap();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn test_parentheses() {
        let mut lexer = Lexer::new("(2 + 3) * 4");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluate(&ast).unwrap();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_power() {
        let mut lexer = Lexer::new("2 ^ 3");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluate(&ast).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_unary_minus() {
        let mut lexer = Lexer::new("-5 + 3");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let result = evaluate(&ast).unwrap();
        assert_eq!(result, -2.0);
    }
}
