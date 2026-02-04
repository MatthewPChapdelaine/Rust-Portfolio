// REST API Client with all HTTP methods
//
// COMPILE & RUN:
//   rustc api_client.rs && ./api_client
//
// For production use with real HTTP requests, add to Cargo.toml:
//   [dependencies]
//   reqwest = { version = "0.11", features = ["blocking", "json"] }
//   serde = { version = "1.0", features = ["derive"] }
//   serde_json = "1.0"
//
// This program demonstrates a REST API client with all HTTP methods

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::time::{Duration, Instant};

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug)]
enum ApiError {
    NetworkError(String),
    ParseError(String),
    ValidationError(String),
    HttpError(u16, String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApiError::HttpError(code, msg) => write!(f, "HTTP {} error: {}", code, msg),
        }
    }
}

impl Error for ApiError {}

// ============================================================================
// HTTP STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
        }
    }
}

#[derive(Debug, Clone)]
struct HttpRequest {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout: Duration,
}

impl HttpRequest {
    fn new(method: HttpMethod, url: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "RustApiClient/1.0".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());

        HttpRequest {
            method,
            url: url.to_string(),
            headers,
            body: None,
            timeout: Duration::from_secs(30),
        }
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    fn json_body(mut self, body: &str) -> Self {
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        self.body = Some(body.to_string());
        self
    }

    fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[derive(Debug, Clone)]
struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
    elapsed: Duration,
}

impl HttpResponse {
    fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    fn is_redirect(&self) -> bool {
        self.status_code >= 300 && self.status_code < 400
    }

    fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }

    fn is_server_error(&self) -> bool {
        self.status_code >= 500
    }
}

// ============================================================================
// API CLIENT
// ============================================================================

struct ApiClient {
    base_url: String,
    default_headers: HashMap<String, String>,
    timeout: Duration,
}

impl ApiClient {
    fn new(base_url: &str) -> Self {
        ApiClient {
            base_url: base_url.to_string(),
            default_headers: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }

    fn with_auth_token(mut self, token: &str) -> Self {
        self.default_headers
            .insert("Authorization".to_string(), format!("Bearer {}", token));
        self
    }

    fn with_header(mut self, key: &str, value: &str) -> Self {
        self.default_headers.insert(key.to_string(), value.to_string());
        self
    }

    fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Execute HTTP request
    fn execute(&self, mut request: HttpRequest) -> Result<HttpResponse, ApiError> {
        // Merge default headers
        for (key, value) in &self.default_headers {
            request.headers.entry(key.clone()).or_insert(value.clone());
        }

        // Build full URL
        let full_url = if request.url.starts_with("http") {
            request.url.clone()
        } else {
            format!("{}{}", self.base_url, request.url)
        };
        request.url = full_url;

        // Execute request (mock implementation)
        self.execute_mock(request)
    }

    /// Mock HTTP execution for demonstration
    fn execute_mock(&self, request: HttpRequest) -> Result<HttpResponse, ApiError> {
        println!("→ {} {}", request.method, request.url);
        
        let start = Instant::now();
        
        // Simulate network delay
        std::thread::sleep(Duration::from_millis(100));

        let (status_code, status_text, body) = match request.method {
            HttpMethod::GET => {
                if request.url.contains("/users/1") {
                    (200, "OK", r#"{"id": 1, "name": "Alice", "email": "alice@example.com"}"#)
                } else if request.url.contains("/users") {
                    (200, "OK", r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#)
                } else if request.url.contains("/notfound") {
                    (404, "Not Found", r#"{"error": "Resource not found"}"#)
                } else {
                    (200, "OK", r#"{"status": "success"}"#)
                }
            }
            HttpMethod::POST => {
                (201, "Created", r#"{"id": 3, "name": "Charlie", "created": true}"#)
            }
            HttpMethod::PUT => {
                (200, "OK", r#"{"id": 1, "name": "Alice Updated", "updated": true}"#)
            }
            HttpMethod::PATCH => {
                (200, "OK", r#"{"id": 1, "name": "Alice Patched", "updated": true}"#)
            }
            HttpMethod::DELETE => {
                (204, "No Content", "")
            }
            HttpMethod::HEAD => {
                (200, "OK", "")
            }
            HttpMethod::OPTIONS => {
                (200, "OK", "")
            }
        };

        let elapsed = start.elapsed();
        
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("server".to_string(), "MockServer/1.0".to_string());

        Ok(HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body: body.to_string(),
            elapsed,
        })
    }

    // ========================================================================
    // CONVENIENCE METHODS
    // ========================================================================

    /// GET request
    fn get(&self, path: &str) -> Result<HttpResponse, ApiError> {
        let request = HttpRequest::new(HttpMethod::GET, path);
        self.execute(request)
    }

    /// POST request
    fn post(&self, path: &str, body: &str) -> Result<HttpResponse, ApiError> {
        let request = HttpRequest::new(HttpMethod::POST, path).json_body(body);
        self.execute(request)
    }

    /// PUT request
    fn put(&self, path: &str, body: &str) -> Result<HttpResponse, ApiError> {
        let request = HttpRequest::new(HttpMethod::PUT, path).json_body(body);
        self.execute(request)
    }

    /// PATCH request
    fn patch(&self, path: &str, body: &str) -> Result<HttpResponse, ApiError> {
        let request = HttpRequest::new(HttpMethod::PATCH, path).json_body(body);
        self.execute(request)
    }

    /// DELETE request
    fn delete(&self, path: &str) -> Result<HttpResponse, ApiError> {
        let request = HttpRequest::new(HttpMethod::DELETE, path);
        self.execute(request)
    }

    /// HEAD request
    fn head(&self, path: &str) -> Result<HttpResponse, ApiError> {
        let request = HttpRequest::new(HttpMethod::HEAD, path);
        self.execute(request)
    }

    /// OPTIONS request
    fn options(&self, path: &str) -> Result<HttpResponse, ApiError> {
        let request = HttpRequest::new(HttpMethod::OPTIONS, path);
        self.execute(request)
    }
}

// ============================================================================
// REQUEST BUILDER
// ============================================================================

struct RequestBuilder<'a> {
    client: &'a ApiClient,
    request: HttpRequest,
}

impl<'a> RequestBuilder<'a> {
    fn new(client: &'a ApiClient, method: HttpMethod, path: &str) -> Self {
        RequestBuilder {
            client,
            request: HttpRequest::new(method, path),
        }
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.request = self.request.header(key, value);
        self
    }

    fn json(mut self, body: &str) -> Self {
        self.request = self.request.json_body(body);
        self
    }

    fn timeout(mut self, timeout: Duration) -> Self {
        self.request = self.request.timeout(timeout);
        self
    }

    fn send(self) -> Result<HttpResponse, ApiError> {
        self.client.execute(self.request)
    }
}

// ============================================================================
// RESPONSE HANDLER
// ============================================================================

struct ResponseHandler;

impl ResponseHandler {
    fn print_response(response: &HttpResponse) {
        println!("← {} {} ({:.0?})", 
            response.status_code, 
            response.status_text,
            response.elapsed
        );
        
        if !response.body.is_empty() {
            println!("  Body: {}", response.body);
        }
    }

    fn extract_json_field(json: &str, field: &str) -> Option<String> {
        // Simple JSON field extraction (for demo purposes)
        let search = format!("\"{}\":", field);
        if let Some(start) = json.find(&search) {
            let value_start = start + search.len();
            let remaining = &json[value_start..].trim_start();
            
            if remaining.starts_with('"') {
                // String value
                if let Some(end) = remaining[1..].find('"') {
                    return Some(remaining[1..end + 1].to_string());
                }
            } else {
                // Number or boolean
                let end = remaining
                    .find(|c: char| c == ',' || c == '}' || c == ']')
                    .unwrap_or(remaining.len());
                return Some(remaining[..end].trim().to_string());
            }
        }
        None
    }
}

// ============================================================================
// DEMO AND EXAMPLES
// ============================================================================

fn demo_basic_requests() {
    println!("=== Basic REST API Operations ===\n");

    let client = ApiClient::new("https://api.example.com");

    // GET request
    println!("1. GET Request:");
    match client.get("/users/1") {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }

    // GET list
    println!("\n2. GET List:");
    match client.get("/users") {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }

    // POST request
    println!("\n3. POST Request:");
    let new_user = r#"{"name": "Charlie", "email": "charlie@example.com"}"#;
    match client.post("/users", new_user) {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }

    // PUT request
    println!("\n4. PUT Request:");
    let updated_user = r#"{"name": "Alice Updated", "email": "alice.new@example.com"}"#;
    match client.put("/users/1", updated_user) {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }

    // PATCH request
    println!("\n5. PATCH Request:");
    let patch_data = r#"{"name": "Alice Patched"}"#;
    match client.patch("/users/1", patch_data) {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }

    // DELETE request
    println!("\n6. DELETE Request:");
    match client.delete("/users/1") {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }

    // HEAD request
    println!("\n7. HEAD Request:");
    match client.head("/users") {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }

    // OPTIONS request
    println!("\n8. OPTIONS Request:");
    match client.options("/users") {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }
}

fn demo_authentication() {
    println!("\n=== Authentication Demo ===\n");

    let client = ApiClient::new("https://api.example.com")
        .with_auth_token("abc123xyz456");

    println!("GET with auth token:");
    match client.get("/protected/data") {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }
}

fn demo_custom_headers() {
    println!("\n=== Custom Headers Demo ===\n");

    let client = ApiClient::new("https://api.example.com")
        .with_header("X-API-Key", "my-secret-key")
        .with_header("X-Custom-Header", "custom-value");

    println!("GET with custom headers:");
    match client.get("/data") {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }
}

fn demo_error_handling() {
    println!("\n=== Error Handling Demo ===\n");

    let client = ApiClient::new("https://api.example.com");

    println!("Requesting non-existent resource:");
    match client.get("/notfound") {
        Ok(response) => {
            ResponseHandler::print_response(&response);
            if response.is_client_error() {
                println!("  ⚠ Client error detected!");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn demo_response_parsing() {
    println!("\n=== Response Parsing Demo ===\n");

    let client = ApiClient::new("https://api.example.com");

    match client.get("/users/1") {
        Ok(response) => {
            ResponseHandler::print_response(&response);
            
            println!("\n  Extracted fields:");
            if let Some(id) = ResponseHandler::extract_json_field(&response.body, "id") {
                println!("    id: {}", id);
            }
            if let Some(name) = ResponseHandler::extract_json_field(&response.body, "name") {
                println!("    name: {}", name);
            }
            if let Some(email) = ResponseHandler::extract_json_field(&response.body, "email") {
                println!("    email: {}", email);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn demo_request_builder() {
    println!("\n=== Request Builder Demo ===\n");

    let client = ApiClient::new("https://api.example.com");

    println!("Using request builder:");
    let request = RequestBuilder::new(&client, HttpMethod::POST, "/users")
        .header("X-Custom", "value")
        .json(r#"{"name": "Dave", "email": "dave@example.com"}"#)
        .timeout(Duration::from_secs(10))
        .send();

    match request {
        Ok(response) => ResponseHandler::print_response(&response),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    demo_basic_requests();
    demo_authentication();
    demo_custom_headers();
    demo_error_handling();
    demo_response_parsing();
    demo_request_builder();

    println!("\n=== Demo Complete ===");
    println!("\nNote: This is a mock implementation for demonstration.");
    println!("For production use, integrate with reqwest crate for real HTTP requests.");
}
