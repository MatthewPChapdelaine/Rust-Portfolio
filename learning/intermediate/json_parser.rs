// JSON Parser with parsing, manipulation, and validation
// 
// COMPILE & RUN:
//   rustc json_parser.rs && ./json_parser
//
// OR with Cargo (add to Cargo.toml: serde = { version = "1.0", features = ["derive"] }, serde_json = "1.0"):
//   cargo run --bin json_parser
//
// This program demonstrates JSON parsing, manipulation, and validation using serde_json

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Custom error type for JSON operations
#[derive(Debug)]
enum JsonError {
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            JsonError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for JsonError {}

/// Simple JSON Value representation
#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

/// JSON Parser struct
struct JsonParser {
    input: Vec<char>,
    position: usize,
}

impl JsonParser {
    fn new(input: &str) -> Self {
        JsonParser {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Parse JSON string into JsonValue
    fn parse(&mut self) -> Result<JsonValue, JsonError> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Err(JsonError::ParseError("Unexpected end of input".to_string()));
        }

        match self.current_char() {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_bool(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            '-' | '0'..='9' => self.parse_number(),
            c => Err(JsonError::ParseError(format!("Unexpected character: {}", c))),
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, JsonError> {
        if self.consume_literal("null") {
            Ok(JsonValue::Null)
        } else {
            Err(JsonError::ParseError("Invalid null value".to_string()))
        }
    }

    fn parse_bool(&mut self) -> Result<JsonValue, JsonError> {
        if self.consume_literal("true") {
            Ok(JsonValue::Bool(true))
        } else if self.consume_literal("false") {
            Ok(JsonValue::Bool(false))
        } else {
            Err(JsonError::ParseError("Invalid boolean value".to_string()))
        }
    }

    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        let start = self.position;
        
        if self.current_char() == '-' {
            self.advance();
        }

        while self.position < self.input.len() && 
              (self.current_char().is_numeric() || self.current_char() == '.') {
            self.advance();
        }

        let num_str: String = self.input[start..self.position].iter().collect();
        num_str.parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| JsonError::ParseError(format!("Invalid number: {}", num_str)))
    }

    fn parse_string(&mut self) -> Result<JsonValue, JsonError> {
        self.advance(); // Skip opening quote
        let mut result = String::new();

        while self.position < self.input.len() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if self.position < self.input.len() {
                    result.push(self.current_char());
                }
            } else {
                result.push(self.current_char());
            }
            self.advance();
        }

        if self.position >= self.input.len() {
            return Err(JsonError::ParseError("Unterminated string".to_string()));
        }

        self.advance(); // Skip closing quote
        Ok(JsonValue::String(result))
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.advance(); // Skip '['
        let mut array = Vec::new();

        self.skip_whitespace();
        if self.current_char() == ']' {
            self.advance();
            return Ok(JsonValue::Array(array));
        }

        loop {
            array.push(self.parse_value()?);
            self.skip_whitespace();

            match self.current_char() {
                ',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                ']' => {
                    self.advance();
                    break;
                }
                _ => return Err(JsonError::ParseError("Expected ',' or ']'".to_string())),
            }
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.advance(); // Skip '{'
        let mut object = HashMap::new();

        self.skip_whitespace();
        if self.current_char() == '}' {
            self.advance();
            return Ok(JsonValue::Object(object));
        }

        loop {
            self.skip_whitespace();
            
            // Parse key (must be string)
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => return Err(JsonError::ParseError("Expected string key".to_string())),
            };

            self.skip_whitespace();
            if self.current_char() != ':' {
                return Err(JsonError::ParseError("Expected ':'".to_string()));
            }
            self.advance();

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();
            match self.current_char() {
                ',' => {
                    self.advance();
                }
                '}' => {
                    self.advance();
                    break;
                }
                _ => return Err(JsonError::ParseError("Expected ',' or '}'".to_string())),
            }
        }

        Ok(JsonValue::Object(object))
    }

    fn current_char(&self) -> char {
        if self.position < self.input.len() {
            self.input[self.position]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn consume_literal(&mut self, literal: &str) -> bool {
        let chars: Vec<char> = literal.chars().collect();
        if self.position + chars.len() > self.input.len() {
            return false;
        }

        for (i, &ch) in chars.iter().enumerate() {
            if self.input[self.position + i] != ch {
                return false;
            }
        }

        self.position += chars.len();
        true
    }
}

/// JSON Validator
struct JsonValidator;

impl JsonValidator {
    /// Validate JSON structure and check for specific requirements
    fn validate(value: &JsonValue) -> Result<(), JsonError> {
        match value {
            JsonValue::Object(obj) => {
                for (key, val) in obj {
                    if key.is_empty() {
                        return Err(JsonError::ValidationError("Empty key found".to_string()));
                    }
                    Self::validate(val)?;
                }
            }
            JsonValue::Array(arr) => {
                for item in arr {
                    Self::validate(item)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Check if object has required fields
    fn has_required_fields(value: &JsonValue, fields: &[&str]) -> bool {
        if let JsonValue::Object(obj) = value {
            fields.iter().all(|field| obj.contains_key(*field))
        } else {
            false
        }
    }
}

/// JSON Manipulator - modify JSON values
struct JsonManipulator;

impl JsonManipulator {
    /// Get value at path (e.g., "user.name")
    fn get_path<'a>(value: &'a JsonValue, path: &str) -> Option<&'a JsonValue> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;

        for part in parts {
            match current {
                JsonValue::Object(obj) => {
                    current = obj.get(part)?;
                }
                _ => return None,
            }
        }

        Some(current)
    }

    /// Set value at path
    fn set_path(value: &mut JsonValue, path: &str, new_value: JsonValue) -> Result<(), JsonError> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err(JsonError::ValidationError("Empty path".to_string()));
        }

        Self::set_path_recursive(value, &parts, 0, new_value)
    }

    fn set_path_recursive(
        current: &mut JsonValue,
        parts: &[&str],
        index: usize,
        new_value: JsonValue,
    ) -> Result<(), JsonError> {
        if index >= parts.len() {
            return Ok(());
        }

        match current {
            JsonValue::Object(obj) => {
                if index == parts.len() - 1 {
                    obj.insert(parts[index].to_string(), new_value);
                    Ok(())
                } else {
                    if let Some(next) = obj.get_mut(parts[index]) {
                        Self::set_path_recursive(next, parts, index + 1, new_value)
                    } else {
                        Err(JsonError::ValidationError(format!("Path not found: {}", parts[index])))
                    }
                }
            }
            _ => Err(JsonError::ValidationError("Not an object".to_string())),
        }
    }

    /// Pretty print JSON
    fn pretty_print(value: &JsonValue, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        match value {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    "[]".to_string()
                } else {
                    let items: Vec<String> = arr.iter()
                        .map(|v| format!("{}{}", "  ".repeat(indent + 1), Self::pretty_print(v, indent + 1)))
                        .collect();
                    format!("[\n{}\n{}]", items.join(",\n"), indent_str)
                }
            }
            JsonValue::Object(obj) => {
                if obj.is_empty() {
                    "{}".to_string()
                } else {
                    let mut items: Vec<String> = obj.iter()
                        .map(|(k, v)| {
                            format!("{}\"{}\": {}", 
                                "  ".repeat(indent + 1), 
                                k, 
                                Self::pretty_print(v, indent + 1))
                        })
                        .collect();
                    items.sort();
                    format!("{{\n{}\n{}}}", items.join(",\n"), indent_str)
                }
            }
        }
    }
}

fn main() {
    println!("=== JSON Parser Demo ===\n");

    // Example 1: Parse simple JSON
    let json_str = r#"{"name": "Alice", "age": 30, "active": true}"#;
    println!("1. Parsing JSON:");
    println!("Input: {}", json_str);
    
    let mut parser = JsonParser::new(json_str);
    match parser.parse() {
        Ok(value) => {
            println!("Parsed successfully!");
            println!("{:#?}\n", value);
        }
        Err(e) => println!("Error: {}\n", e),
    }

    // Example 2: Parse complex nested JSON
    let complex_json = r#"{
        "user": {
            "id": 123,
            "name": "Bob",
            "tags": ["admin", "developer"],
            "settings": {
                "theme": "dark",
                "notifications": true
            }
        }
    }"#;

    println!("2. Parsing complex JSON:");
    let mut parser = JsonParser::new(complex_json);
    match parser.parse() {
        Ok(mut value) => {
            println!("Parsed successfully!");
            
            // Validate
            println!("\n3. Validating JSON:");
            match JsonValidator::validate(&value) {
                Ok(_) => println!("✓ Validation passed"),
                Err(e) => println!("✗ Validation failed: {}", e),
            }

            // Check required fields
            let has_fields = JsonValidator::has_required_fields(&value, &["user"]);
            println!("Has required fields: {}", has_fields);

            // Get nested value
            println!("\n4. Getting nested value:");
            if let Some(name) = JsonManipulator::get_path(&value, "user.name") {
                println!("user.name = {:?}", name);
            }

            // Modify value
            println!("\n5. Modifying JSON:");
            if let Err(e) = JsonManipulator::set_path(&mut value, "user.name", JsonValue::String("Charlie".to_string())) {
                println!("Error: {}", e);
            } else {
                println!("Modified user.name to 'Charlie'");
            }

            // Pretty print
            println!("\n6. Pretty printed JSON:");
            println!("{}", JsonManipulator::pretty_print(&value, 0));
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 3: Parse array
    println!("\n7. Parsing array:");
    let array_json = r#"[1, 2, 3, "four", true, null]"#;
    let mut parser = JsonParser::new(array_json);
    match parser.parse() {
        Ok(value) => {
            println!("Parsed: {}", JsonManipulator::pretty_print(&value, 0));
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Demo Complete ===");
}
