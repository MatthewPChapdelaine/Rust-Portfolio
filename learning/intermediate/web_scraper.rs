// Web Scraper with HTTP client, HTML parsing, and retry logic
//
// COMPILE & RUN (requires dependencies):
//   Add to Cargo.toml:
//     [dependencies]
//     reqwest = { version = "0.11", features = ["blocking"] }
//     scraper = "0.17"
//     tokio = { version = "1", features = ["full"] }
//
//   Then run: cargo run --bin web_scraper
//
// SIMPLE STANDALONE VERSION (no external crates):
//   rustc web_scraper.rs && ./web_scraper
//
// This program demonstrates HTTP client usage, HTML parsing, and retry mechanisms

use std::error::Error;
use std::fmt;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

/// Custom error type for web scraping operations
#[derive(Debug)]
enum ScraperError {
    NetworkError(String),
    ParseError(String),
    RetryExhausted(String),
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScraperError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ScraperError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ScraperError::RetryExhausted(msg) => write!(f, "Retry exhausted: {}", msg),
        }
    }
}

impl Error for ScraperError {}

/// HTTP method enum
#[derive(Debug, Clone)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
        }
    }
}

/// HTTP Request builder
struct HttpRequest {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpRequest {
    fn new(method: HttpMethod, url: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "RustScraper/1.0".to_string());
        
        HttpRequest {
            method,
            url: url.to_string(),
            headers,
            body: None,
        }
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
}

/// HTTP Response
#[derive(Debug, Clone)]
struct HttpResponse {
    status_code: u16,
    body: String,
    headers: HashMap<String, String>,
}

impl HttpResponse {
    fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}

/// Retry configuration
#[derive(Clone)]
struct RetryConfig {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

/// Simple HTTP Client with retry logic (mock implementation)
struct HttpClient {
    retry_config: RetryConfig,
}

impl HttpClient {
    fn new() -> Self {
        HttpClient {
            retry_config: RetryConfig::default(),
        }
    }

    fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Execute request with retry logic
    fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, ScraperError> {
        let mut attempt = 0;
        let mut delay = self.retry_config.initial_delay;

        loop {
            attempt += 1;
            println!("  Attempt {} of {}", attempt, self.retry_config.max_attempts);

            match self.execute_once(request) {
                Ok(response) => {
                    if response.is_success() {
                        return Ok(response);
                    } else if attempt >= self.retry_config.max_attempts {
                        return Err(ScraperError::RetryExhausted(
                            format!("Failed after {} attempts: Status {}", attempt, response.status_code)
                        ));
                    }
                }
                Err(e) => {
                    if attempt >= self.retry_config.max_attempts {
                        return Err(e);
                    }
                }
            }

            println!("  Retrying in {:?}...", delay);
            thread::sleep(delay);

            // Exponential backoff
            delay = Duration::from_millis(
                ((delay.as_millis() as f64) * self.retry_config.backoff_multiplier) as u64
            ).min(self.retry_config.max_delay);
        }
    }

    /// Execute single request attempt (mock implementation)
    fn execute_once(&self, request: &HttpRequest) -> Result<HttpResponse, ScraperError> {
        println!("  {} {}", request.method, request.url);
        
        // Simulate network request
        // In real implementation, would use actual HTTP library
        
        // Mock response based on URL
        if request.url.contains("example.com") {
            Ok(HttpResponse {
                status_code: 200,
                body: Self::mock_html_content(),
                headers: HashMap::new(),
            })
        } else if request.url.contains("api.example.com") {
            Ok(HttpResponse {
                status_code: 200,
                body: r#"{"data": "mock response"}"#.to_string(),
                headers: HashMap::new(),
            })
        } else {
            Err(ScraperError::NetworkError("Unknown host".to_string()))
        }
    }

    fn mock_html_content() -> String {
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Example Page</title>
</head>
<body>
    <div id="content">
        <h1>Main Title</h1>
        <p class="description">This is a sample paragraph.</p>
        <ul class="items">
            <li><a href="/page1">Link 1</a></li>
            <li><a href="/page2">Link 2</a></li>
            <li><a href="/page3">Link 3</a></li>
        </ul>
        <div class="data-item" data-id="1">Item One</div>
        <div class="data-item" data-id="2">Item Two</div>
    </div>
</body>
</html>"#.to_string()
    }
}

/// Simple HTML Parser
struct HtmlParser {
    content: String,
}

impl HtmlParser {
    fn new(content: String) -> Self {
        HtmlParser { content }
    }

    /// Extract text between tags
    fn extract_tag_content(&self, tag: &str) -> Vec<String> {
        let mut results = Vec::new();
        let open_tag = format!("<{}", tag);
        let close_tag = format!("</{}>", tag);

        let mut pos = 0;
        while let Some(start) = self.content[pos..].find(&open_tag) {
            let start = pos + start;
            if let Some(tag_end) = self.content[start..].find('>') {
                let content_start = start + tag_end + 1;
                if let Some(end) = self.content[content_start..].find(&close_tag) {
                    let content = self.content[content_start..content_start + end].trim().to_string();
                    results.push(content);
                    pos = content_start + end + close_tag.len();
                    continue;
                }
            }
            break;
        }

        results
    }

    /// Extract all links (href attributes)
    fn extract_links(&self) -> Vec<String> {
        let mut links = Vec::new();
        let mut pos = 0;

        while let Some(href_pos) = self.content[pos..].find("href=\"") {
            let start = pos + href_pos + 6;
            if let Some(end) = self.content[start..].find('"') {
                let link = self.content[start..start + end].to_string();
                links.push(link);
                pos = start + end;
            } else {
                break;
            }
        }

        links
    }

    /// Extract elements by class name
    fn extract_by_class(&self, class_name: &str) -> Vec<String> {
        let mut results = Vec::new();
        let class_pattern = format!("class=\"{}\"", class_name);
        let mut pos = 0;

        while let Some(class_pos) = self.content[pos..].find(&class_pattern) {
            let start = pos + class_pos;
            // Find the opening tag
            if let Some(tag_start) = self.content[..start].rfind('<') {
                if let Some(tag_name_end) = self.content[tag_start..start].find(' ') {
                    let tag_name = &self.content[tag_start + 1..tag_start + tag_name_end];
                    let close_tag = format!("</{}>", tag_name);
                    
                    if let Some(tag_end) = self.content[start..].find('>') {
                        let content_start = start + tag_end + 1;
                        if let Some(end) = self.content[content_start..].find(&close_tag) {
                            let content = self.content[content_start..content_start + end].trim().to_string();
                            results.push(content);
                            pos = content_start + end;
                            continue;
                        }
                    }
                }
            }
            pos = start + 1;
        }

        results
    }

    /// Extract text content only (strip HTML tags)
    fn extract_text(&self) -> String {
        let mut result = String::new();
        let mut in_tag = false;

        for ch in self.content.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ => {
                    if !in_tag && !ch.is_whitespace() {
                        result.push(ch);
                    } else if !in_tag && ch == ' ' {
                        if !result.ends_with(' ') {
                            result.push(' ');
                        }
                    }
                }
            }
        }

        result.trim().to_string()
    }
}

/// Web Scraper that combines HTTP client and HTML parser
struct WebScraper {
    client: HttpClient,
}

impl WebScraper {
    fn new() -> Self {
        WebScraper {
            client: HttpClient::new(),
        }
    }

    fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.client = self.client.with_retry_config(config);
        self
    }

    /// Scrape a URL and parse the response
    fn scrape(&self, url: &str) -> Result<HtmlParser, ScraperError> {
        let request = HttpRequest::new(HttpMethod::GET, url);
        let response = self.client.execute(&request)?;
        
        if response.is_success() {
            Ok(HtmlParser::new(response.body))
        } else {
            Err(ScraperError::NetworkError(
                format!("HTTP {}", response.status_code)
            ))
        }
    }

    /// Scrape multiple URLs concurrently (simulated)
    fn scrape_multiple(&self, urls: &[&str]) -> Vec<Result<HtmlParser, ScraperError>> {
        urls.iter().map(|url| self.scrape(url)).collect()
    }
}

/// Data extraction result
#[derive(Debug)]
struct ScrapedData {
    url: String,
    title: Option<String>,
    links: Vec<String>,
    paragraphs: Vec<String>,
}

fn main() {
    println!("=== Web Scraper Demo ===\n");

    // Example 1: Basic scraping
    println!("1. Basic Web Scraping:");
    let scraper = WebScraper::new();
    
    match scraper.scrape("https://example.com") {
        Ok(parser) => {
            println!("✓ Successfully fetched page\n");

            // Extract titles
            let titles = parser.extract_tag_content("h1");
            println!("Titles found: {}", titles.len());
            for title in &titles {
                println!("  - {}", title);
            }

            // Extract links
            println!("\nLinks found:");
            let links = parser.extract_links();
            for link in &links {
                println!("  - {}", link);
            }

            // Extract by class
            println!("\nElements with class 'data-item':");
            let items = parser.extract_by_class("data-item");
            for item in &items {
                println!("  - {}", item);
            }

            // Extract plain text
            println!("\nPlain text content (first 100 chars):");
            let text = parser.extract_text();
            println!("  {}", &text.chars().take(100).collect::<String>());
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 2: Scraping with custom retry config
    println!("\n2. Scraping with Custom Retry Configuration:");
    let retry_config = RetryConfig {
        max_attempts: 5,
        initial_delay: Duration::from_millis(500),
        max_delay: Duration::from_secs(5),
        backoff_multiplier: 1.5,
    };
    
    let scraper = WebScraper::new().with_retry_config(retry_config);
    
    match scraper.scrape("https://api.example.com/data") {
        Ok(_) => println!("✓ Successfully fetched API data"),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 3: Multiple URL scraping
    println!("\n3. Scraping Multiple URLs:");
    let urls = vec![
        "https://example.com/page1",
        "https://example.com/page2",
        "https://example.com/page3",
    ];

    let results = scraper.scrape_multiple(&urls);
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(_) => println!("  ✓ URL {} scraped successfully", i + 1),
            Err(e) => println!("  ✗ URL {} failed: {}", i + 1, e),
        }
    }

    // Example 4: Rate limiting demonstration
    println!("\n4. Rate Limiting (simulated delay between requests):");
    let urls_to_scrape = vec!["https://example.com"; 3];
    
    for (i, url) in urls_to_scrape.iter().enumerate() {
        println!("  Request {}/{}:", i + 1, urls_to_scrape.len());
        let _ = scraper.scrape(url);
        
        if i < urls_to_scrape.len() - 1 {
            println!("  Sleeping 1s to respect rate limits...");
            thread::sleep(Duration::from_secs(1));
        }
    }

    println!("\n=== Demo Complete ===");
    println!("\nNote: This is a mock implementation for demonstration.");
    println!("For production use, integrate with reqwest and scraper crates.");
}
