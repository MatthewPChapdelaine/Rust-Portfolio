// Complete Interpreter with Lexer, Parser, AST, Symbol Tables, and REPL
// Implements a simple expression language with variables, functions, and control flow

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

// ========== TOKEN DEFINITIONS ==========
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Assign,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    If,
    Else,
    While,
    Fn,
    Return,
    Comma,
    Semicolon,
    Eof,
}

// ========== LEXER ==========
struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let current_char = input.chars().next();
        Lexer {
            input,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
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

    fn read_number(&mut self) -> f64 {
        let start = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_numeric() || ch == '.' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].parse().unwrap()
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            None => Token::Eof,
            Some(ch) => {
                if ch.is_numeric() {
                    return Token::Number(self.read_number());
                }
                if ch.is_alphabetic() {
                    let ident = self.read_identifier();
                    return match ident.as_str() {
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "fn" => Token::Fn,
                        "return" => Token::Return,
                        _ => Token::Identifier(ident),
                    };
                }

                let token = match ch {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Star,
                    '/' => Token::Slash,
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    '{' => Token::LBrace,
                    '}' => Token::RBrace,
                    ',' => Token::Comma,
                    ';' => Token::Semicolon,
                    '=' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            Token::Equal
                        } else {
                            self.position -= 1;
                            self.current_char = Some('=');
                            Token::Assign
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current_char == Some('=') {
                            Token::NotEqual
                        } else {
                            panic!("Unexpected character: !")
                        }
                    }
                    '<' => Token::LessThan,
                    '>' => Token::GreaterThan,
                    _ => panic!("Unexpected character: {}", ch),
                };
                self.advance();
                token
            }
        }
    }
}

// ========== AST DEFINITIONS ==========
#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
enum Stmt {
    Assign {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Expr),
    Expr(Expr),
}

// ========== PARSER ==========
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, token: Token) -> Result<(), String> {
        if self.current() == &token {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", token, self.current()))
        }
    }

    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while self.current() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.current() {
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Fn => self.parse_function(),
            Token::Return => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(expr))
            }
            Token::Identifier(_) => {
                let name = if let Token::Identifier(n) = self.current().clone() {
                    n
                } else {
                    unreachable!()
                };
                self.advance();

                if self.current() == &Token::Assign {
                    self.advance();
                    let value = self.parse_expression()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Stmt::Assign { name, value })
                } else {
                    self.position -= 1;
                    let expr = self.parse_expression()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Stmt::Expr(expr))
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect(Token::If)?;
        self.expect(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;

        let mut then_branch = Vec::new();
        while self.current() != &Token::RBrace {
            then_branch.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        let else_branch = if self.current() == &Token::Else {
            self.advance();
            self.expect(Token::LBrace)?;
            let mut else_stmts = Vec::new();
            while self.current() != &Token::RBrace {
                else_stmts.push(self.parse_statement()?);
            }
            self.expect(Token::RBrace)?;
            Some(else_stmts)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;

        let mut body = Vec::new();
        while self.current() != &Token::RBrace {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Fn)?;
        let name = if let Token::Identifier(n) = self.current().clone() {
            n
        } else {
            return Err("Expected function name".to_string());
        };
        self.advance();
        self.expect(Token::LParen)?;

        let mut params = Vec::new();
        while self.current() != &Token::RParen {
            if let Token::Identifier(param) = self.current().clone() {
                params.push(param);
                self.advance();
                if self.current() == &Token::Comma {
                    self.advance();
                }
            } else {
                return Err("Expected parameter name".to_string());
            }
        }
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;

        let mut body = Vec::new();
        while self.current() != &Token::RBrace {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        Ok(Stmt::Function { name, params, body })
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while matches!(
            self.current(),
            Token::Equal | Token::NotEqual | Token::LessThan | Token::GreaterThan
        ) {
            let op = match self.current() {
                Token::Equal => BinOp::Equal,
                Token::NotEqual => BinOp::NotEqual,
                Token::LessThan => BinOp::LessThan,
                Token::GreaterThan => BinOp::GreaterThan,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = match self.current() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while matches!(self.current(), Token::Star | Token::Slash) {
            let op = match self.current() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_primary()?;
            left = Expr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::Identifier(name) => {
                self.advance();
                if self.current() == &Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current() != &Token::RParen {
                        args.push(self.parse_expression()?);
                        if self.current() == &Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current())),
        }
    }
}

// ========== INTERPRETER ==========
#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    Function { params: Vec<String>, body: Vec<Stmt> },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Function { .. } => write!(f, "<function>"),
        }
    }
}

struct Interpreter<'a> {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
    return_value: Option<Value>,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> Interpreter<'a> {
    fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            locals: Vec::new(),
            return_value: None,
            _lifetime: std::marker::PhantomData,
        }
    }

    fn get_variable(&self, name: &str) -> Result<Value, String> {
        for scope in self.locals.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        self.globals
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable: {}", name))
    }

    fn set_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Variable(name) => self.get_variable(name),
            Expr::BinaryOp { op, left, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        let result = match op {
                            BinOp::Add => l + r,
                            BinOp::Sub => l - r,
                            BinOp::Mul => l * r,
                            BinOp::Div => l / r,
                            BinOp::Equal => {
                                if (l - r).abs() < f64::EPSILON {
                                    1.0
                                } else {
                                    0.0
                                }
                            }
                            BinOp::NotEqual => {
                                if (l - r).abs() >= f64::EPSILON {
                                    1.0
                                } else {
                                    0.0
                                }
                            }
                            BinOp::LessThan => {
                                if l < r {
                                    1.0
                                } else {
                                    0.0
                                }
                            }
                            BinOp::GreaterThan => {
                                if l > r {
                                    1.0
                                } else {
                                    0.0
                                }
                            }
                        };
                        Ok(Value::Number(result))
                    }
                    _ => Err("Type error in binary operation".to_string()),
                }
            }
            Expr::Call { name, args } => {
                let func = self.get_variable(name)?;
                if let Value::Function { params, body } = func {
                    if args.len() != params.len() {
                        return Err(format!(
                            "Wrong number of arguments: expected {}, got {}",
                            params.len(),
                            args.len()
                        ));
                    }

                    let mut arg_values = Vec::new();
                    for arg in args {
                        arg_values.push(self.eval_expr(arg)?);
                    }

                    self.locals.push(HashMap::new());
                    for (param, value) in params.iter().zip(arg_values) {
                        self.set_variable(param.clone(), value);
                    }

                    for stmt in &body {
                        self.eval_stmt(stmt)?;
                        if self.return_value.is_some() {
                            break;
                        }
                    }

                    let result = self.return_value.take().unwrap_or(Value::Number(0.0));
                    self.locals.pop();
                    Ok(result)
                } else {
                    Err(format!("{} is not a function", name))
                }
            }
        }
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.set_variable(name.clone(), val);
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.eval_expr(condition)?;
                if let Value::Number(n) = cond {
                    if n != 0.0 {
                        for stmt in then_branch {
                            self.eval_stmt(stmt)?;
                            if self.return_value.is_some() {
                                break;
                            }
                        }
                    } else if let Some(else_stmts) = else_branch {
                        for stmt in else_stmts {
                            self.eval_stmt(stmt)?;
                            if self.return_value.is_some() {
                                break;
                            }
                        }
                    }
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = self.eval_expr(condition)?;
                    if let Value::Number(n) = cond {
                        if n == 0.0 {
                            break;
                        }
                        for stmt in body {
                            self.eval_stmt(stmt)?;
                            if self.return_value.is_some() {
                                return Ok(());
                            }
                        }
                    } else {
                        break;
                    }
                }
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                };
                self.set_variable(name.clone(), func);
                Ok(())
            }
            Stmt::Return(expr) => {
                let value = self.eval_expr(expr)?;
                self.return_value = Some(value);
                Ok(())
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(())
            }
        }
    }

    fn execute(&mut self, program: &[Stmt]) -> Result<Option<Value>, String> {
        let mut last_value = None;
        for stmt in program {
            match stmt {
                Stmt::Expr(expr) => {
                    last_value = Some(self.eval_expr(expr)?);
                }
                _ => {
                    self.eval_stmt(stmt)?;
                }
            }
        }
        Ok(last_value)
    }
}

// ========== REPL ==========
fn repl() {
    let mut interpreter = Interpreter::new();
    println!("Welcome to the Interpreter REPL!");
    println!("Type expressions or statements. Use Ctrl+C to exit.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        let mut parser = Parser::new(tokens);
        match parser.parse_program() {
            Ok(program) => match interpreter.execute(&program) {
                Ok(Some(value)) => println!("{}", value),
                Ok(None) => {}
                Err(e) => println!("Runtime error: {}", e),
            },
            Err(e) => println!("Parse error: {}", e),
        }
    }
}

// ========== MAIN ==========
fn main() {
    println!("=== Compiler/Interpreter Demo ===\n");

    // Example 1: Basic arithmetic
    println!("Example 1: Basic Arithmetic");
    let code1 = "2 + 3 * 4;";
    let mut lexer = Lexer::new(code1);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new();
    if let Ok(Some(result)) = interpreter.execute(&program) {
        println!("{} = {}\n", code1, result);
    }

    // Example 2: Variables
    println!("Example 2: Variables");
    let code2 = "x = 10; y = 20; x + y;";
    let mut lexer = Lexer::new(code2);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new();
    if let Ok(Some(result)) = interpreter.execute(&program) {
        println!("{} = {}\n", code2, result);
    }

    // Example 3: Functions
    println!("Example 3: Functions");
    let code3 = r#"
        fn add(a, b) {
            return a + b;
        }
        add(5, 7);
    "#;
    let mut lexer = Lexer::new(code3);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new();
    if let Ok(Some(result)) = interpreter.execute(&program) {
        println!("add(5, 7) = {}\n", result);
    }

    // Example 4: Conditionals
    println!("Example 4: Conditionals");
    let code4 = r#"
        x = 15;
        if (x > 10) {
            result = 100;
        } else {
            result = 50;
        }
        result;
    "#;
    let mut lexer = Lexer::new(code4);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new();
    if let Ok(Some(result)) = interpreter.execute(&program) {
        println!("Conditional result = {}\n", result);
    }

    // Example 5: Loops
    println!("Example 5: Loops (Factorial)");
    let code5 = r#"
        fn factorial(n) {
            result = 1;
            i = 1;
            while (i < n + 1) {
                result = result * i;
                i = i + 1;
            }
            return result;
        }
        factorial(5);
    "#;
    let mut lexer = Lexer::new(code5);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new();
    if let Ok(Some(result)) = interpreter.execute(&program) {
        println!("factorial(5) = {}\n", result);
    }

    // Example 6: Fibonacci
    println!("Example 6: Fibonacci");
    let code6 = r#"
        fn fib(n) {
            if (n < 2) {
                return n;
            } else {
                a = 0;
                b = 1;
                i = 2;
                while (i < n + 1) {
                    temp = a + b;
                    a = b;
                    b = temp;
                    i = i + 1;
                }
                return b;
            }
        }
        fib(10);
    "#;
    let mut lexer = Lexer::new(code6);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();
    let mut interpreter = Interpreter::new();
    if let Ok(Some(result)) = interpreter.execute(&program) {
        println!("fib(10) = {}\n", result);
    }

    println!("\n=== Starting REPL ===");
    repl();
}
