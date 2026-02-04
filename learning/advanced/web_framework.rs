/*!
 * Mini Web Framework
 * 
 * A minimal web framework featuring:
 * - HTTP server with request parsing
 * - Route matching and handlers
 * - Middleware support
 * - JSON response helpers
 * - Query parameter parsing
 * 
 * # Compile and Run
 * ```bash
 * rustc web_framework.rs -o web_framework
 * ./web_framework
 * ```
 * 
 * # Test with:
 * ```bash
 * curl http://localhost:8080/
 * curl http://localhost:8080/hello/World
 * curl http://localhost:8080/json
 * curl http://localhost:8080/echo?msg=Hello
 * curl -X POST http://localhost:8080/data -d "test data"
 * ```
 */

use std::collections::HashMap;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

// ============================================================================
// HTTP Request Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Method {
    fn from_str(s: &str) -> Option<Method> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "PATCH" => Some(Method::PATCH),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub params: HashMap<String, String>,
}

impl Request {
    /// Parse HTTP request from stream
    fn parse(stream: &mut TcpStream) -> Result<Request, String> {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut lines = Vec::new();
        
        // Read headers
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).map_err(|e| e.to_string())?;
            
            if line == "\r\n" || line == "\n" {
                break;
            }
            lines.push(line.trim().to_string());
        }

        if lines.is_empty() {
            return Err("Empty request".to_string());
        }

        // Parse request line
        let parts: Vec<&str> = lines[0].split_whitespace().collect();
        if parts.len() < 2 {
            return Err("Invalid request line".to_string());
        }

        let method = Method::from_str(parts[0])
            .ok_or_else(|| format!("Unknown method: {}", parts[0]))?;
        
        let (path, query) = Self::parse_path_and_query(parts[1]);

        // Parse headers
        let mut headers = HashMap::new();
        for line in &lines[1..] {
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_lowercase();
                let value = line[pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // Read body if present
        let mut body = String::new();
        if let Some(content_length) = headers.get("content-length") {
            if let Ok(length) = content_length.parse::<usize>() {
                let mut buffer = vec![0; length];
                reader.read_exact(&mut buffer).map_err(|e| e.to_string())?;
                body = String::from_utf8_lossy(&buffer).to_string();
            }
        }

        Ok(Request {
            method,
            path,
            query,
            headers,
            body,
            params: HashMap::new(),
        })
    }

    fn parse_path_and_query(uri: &str) -> (String, HashMap<String, String>) {
        let parts: Vec<&str> = uri.split('?').collect();
        let path = parts[0].to_string();
        let mut query = HashMap::new();

        if parts.len() > 1 {
            for param in parts[1].split('&') {
                let kv: Vec<&str> = param.split('=').collect();
                if kv.len() == 2 {
                    query.insert(
                        kv[0].to_string(),
                        urlencoding::decode(kv[1]).unwrap_or_default().to_string(),
                    );
                }
            }
        }

        (path, query)
    }
}

// Simple URL decoding
mod urlencoding {
    pub fn decode(s: &str) -> Option<String> {
        let s = s.replace('+', " ");
        Some(s.replace("%20", " "))
    }
}

// ============================================================================
// HTTP Response
// ============================================================================

#[derive(Debug)]
pub struct Response {
    status: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new(status: u16, status_text: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        
        Response {
            status,
            status_text: status_text.to_string(),
            headers,
            body: String::new(),
        }
    }

    pub fn ok(body: &str) -> Self {
        let mut resp = Self::new(200, "OK");
        resp.body = body.to_string();
        resp
    }

    pub fn json(body: &str) -> Self {
        let mut resp = Self::ok(body);
        resp.headers.insert("Content-Type".to_string(), "application/json".to_string());
        resp
    }

    pub fn not_found() -> Self {
        let mut resp = Self::new(404, "Not Found");
        resp.body = "404 Not Found".to_string();
        resp
    }

    pub fn internal_error(msg: &str) -> Self {
        let mut resp = Self::new(500, "Internal Server Error");
        resp.body = format!("500 Internal Server Error: {}", msg);
        resp
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status, self.status_text);
        
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        response.push_str("\r\n");
        response.push_str(&self.body);
        
        response.into_bytes()
    }
}

// ============================================================================
// Router and Handlers
// ============================================================================

pub type Handler = Arc<dyn Fn(&mut Request) -> Response + Send + Sync>;
pub type Middleware = Arc<dyn Fn(&mut Request, Handler) -> Response + Send + Sync>;

struct Route {
    method: Method,
    pattern: String,
    handler: Handler,
}

impl Route {
    fn matches(&self, method: &Method, path: &str) -> Option<HashMap<String, String>> {
        if method != &self.method {
            return None;
        }

        let pattern_parts: Vec<&str> = self.pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (pattern, path) in pattern_parts.iter().zip(path_parts.iter()) {
            if pattern.starts_with(':') {
                params.insert(pattern[1..].to_string(), path.to_string());
            } else if pattern != path {
                return None;
            }
        }

        Some(params)
    }
}

pub struct Router {
    routes: Vec<Route>,
    middlewares: Vec<Middleware>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn get<F>(&mut self, pattern: &str, handler: F)
    where
        F: Fn(&mut Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: Method::GET,
            pattern: pattern.to_string(),
            handler: Arc::new(handler),
        });
    }

    pub fn post<F>(&mut self, pattern: &str, handler: F)
    where
        F: Fn(&mut Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: Method::POST,
            pattern: pattern.to_string(),
            handler: Arc::new(handler),
        });
    }

    pub fn use_middleware<F>(&mut self, middleware: F)
    where
        F: Fn(&mut Request, Handler) -> Response + Send + Sync + 'static,
    {
        self.middlewares.push(Arc::new(middleware));
    }

    fn handle(&self, mut request: Request) -> Response {
        // Find matching route
        for route in &self.routes {
            if let Some(params) = route.matches(&request.method, &request.path) {
                request.params = params;
                
                // Apply middlewares
                let handler = route.handler.clone();
                return self.apply_middlewares(request, handler, 0);
            }
        }

        Response::not_found()
    }

    fn apply_middlewares(&self, mut request: Request, handler: Handler, index: usize) -> Response {
        if index >= self.middlewares.len() {
            return handler(&mut request);
        }

        let middleware = self.middlewares[index].clone();
        let next_handler: Handler = Arc::new({
            let router_middlewares = self.middlewares.clone();
            let handler = handler.clone();
            move |req| {
                // Create a mini router for recursive middleware application
                let mut next_index = index + 1;
                if next_index >= router_middlewares.len() {
                    handler(req)
                } else {
                    let mw = router_middlewares[next_index].clone();
                    mw(req, handler.clone())
                }
            }
        });

        middleware(&mut request, next_handler)
    }
}

// ============================================================================
// Web Framework (App)
// ============================================================================

pub struct App {
    router: Arc<Router>,
}

impl App {
    pub fn new(router: Router) -> Self {
        App {
            router: Arc::new(router),
        }
    }

    pub fn listen(&self, addr: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr)?;
        println!("🚀 Server listening on http://{}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let router = self.router.clone();
                    std::thread::spawn(move || {
                        handle_connection(&mut stream, &router);
                    });
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                }
            }
        }

        Ok(())
    }
}

fn handle_connection(stream: &mut TcpStream, router: &Router) {
    let request = match Request::parse(stream) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);
            let response = Response::internal_error(&e);
            let _ = stream.write_all(&response.to_bytes());
            return;
        }
    };

    println!("{} {}", request.method.clone() as u8, request.path);

    let response = router.handle(request);
    
    if let Err(e) = stream.write_all(&response.to_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}

// ============================================================================
// Example Application
// ============================================================================

fn main() {
    let mut router = Router::new();

    // Logging middleware
    router.use_middleware(|req, next| {
        println!("📨 {} {}", format!("{:?}", req.method), req.path);
        let response = next(req);
        println!("📤 Status: {}", response.status);
        response
    });

    // Routes
    router.get("/", |_req| {
        Response::ok("Welcome to Rust Mini Web Framework! Try /hello/YourName or /json")
    });

    router.get("/hello/:name", |req| {
        let name = req.params.get("name").map(|s| s.as_str()).unwrap_or("World");
        Response::ok(&format!("Hello, {}!", name))
    });

    router.get("/json", |_req| {
        Response::json(r#"{"message": "Hello from JSON!", "status": "ok"}"#)
    });

    router.get("/echo", |req| {
        let msg = req.query.get("msg").map(|s| s.as_str()).unwrap_or("No message");
        Response::ok(&format!("Echo: {}", msg))
    });

    router.post("/data", |req| {
        Response::ok(&format!("Received {} bytes: {}", req.body.len(), req.body))
    });

    router.get("/headers", |req| {
        let mut body = String::from("Request Headers:\n");
        for (key, value) in &req.headers {
            body.push_str(&format!("{}: {}\n", key, value));
        }
        Response::ok(&body)
    });

    let app = App::new(router);
    
    if let Err(e) = app.listen("127.0.0.1:8080") {
        eprintln!("Server error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_parsing() {
        assert_eq!(Method::from_str("GET"), Some(Method::GET));
        assert_eq!(Method::from_str("POST"), Some(Method::POST));
        assert_eq!(Method::from_str("INVALID"), None);
    }

    #[test]
    fn test_route_matching() {
        let route = Route {
            method: Method::GET,
            pattern: "/hello/:name".to_string(),
            handler: Arc::new(|_| Response::ok("test")),
        };

        let params = route.matches(&Method::GET, "/hello/world");
        assert!(params.is_some());
        assert_eq!(params.unwrap().get("name"), Some(&"world".to_string()));

        assert!(route.matches(&Method::POST, "/hello/world").is_none());
        assert!(route.matches(&Method::GET, "/goodbye/world").is_none());
    }

    #[test]
    fn test_response_creation() {
        let resp = Response::ok("Hello");
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, "Hello");

        let resp = Response::not_found();
        assert_eq!(resp.status, 404);
    }

    #[test]
    fn test_path_query_parsing() {
        let (path, query) = Request::parse_path_and_query("/test?foo=bar&baz=qux");
        assert_eq!(path, "/test");
        assert_eq!(query.get("foo"), Some(&"bar".to_string()));
        assert_eq!(query.get("baz"), Some(&"qux".to_string()));
    }
}
