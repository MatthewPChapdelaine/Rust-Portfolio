// File Processor: CSV Processing with Statistics and Report Generation
//
// COMPILE & RUN:
//   rustc file_processor.rs && ./file_processor
//
// This program demonstrates CSV file processing, statistical analysis,
// and report generation

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug)]
enum ProcessorError {
    IoError(String),
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessorError::IoError(msg) => write!(f, "IO error: {}", msg),
            ProcessorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProcessorError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for ProcessorError {}

// ============================================================================
// CSV STRUCTURES
// ============================================================================

/// Represents a CSV row
#[derive(Debug, Clone)]
struct CsvRow {
    fields: Vec<String>,
}

impl CsvRow {
    fn new(fields: Vec<String>) -> Self {
        CsvRow { fields }
    }

    fn get(&self, index: usize) -> Option<&String> {
        self.fields.get(index)
    }

    fn len(&self) -> usize {
        self.fields.len()
    }
}

/// CSV Data structure
#[derive(Debug)]
struct CsvData {
    headers: Vec<String>,
    rows: Vec<CsvRow>,
}

impl CsvData {
    fn new() -> Self {
        CsvData {
            headers: Vec::new(),
            rows: Vec::new(),
        }
    }

    fn with_headers(headers: Vec<String>) -> Self {
        CsvData {
            headers,
            rows: Vec::new(),
        }
    }

    fn add_row(&mut self, row: CsvRow) {
        self.rows.push(row);
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn column_count(&self) -> usize {
        self.headers.len()
    }

    /// Get column by name
    fn get_column(&self, column_name: &str) -> Option<Vec<&String>> {
        self.headers
            .iter()
            .position(|h| h == column_name)
            .map(|idx| self.rows.iter().filter_map(|row| row.get(idx)).collect())
    }

    /// Get column by index
    fn get_column_by_index(&self, index: usize) -> Vec<&String> {
        self.rows.iter().filter_map(|row| row.get(index)).collect()
    }
}

// ============================================================================
// CSV PARSER
// ============================================================================

struct CsvParser {
    delimiter: char,
    has_headers: bool,
}

impl CsvParser {
    fn new() -> Self {
        CsvParser {
            delimiter: ',',
            has_headers: true,
        }
    }

    fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    fn with_headers(mut self, has_headers: bool) -> Self {
        self.has_headers = has_headers;
        self
    }

    /// Parse CSV from string
    fn parse_string(&self, content: &str) -> Result<CsvData, ProcessorError> {
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.is_empty() {
            return Ok(CsvData::new());
        }

        let mut csv_data = if self.has_headers {
            let headers = self.parse_line(lines[0]);
            CsvData::with_headers(headers)
        } else {
            CsvData::new()
        };

        let start_idx = if self.has_headers { 1 } else { 0 };
        
        for line in lines.iter().skip(start_idx) {
            if !line.trim().is_empty() {
                let fields = self.parse_line(line);
                csv_data.add_row(CsvRow::new(fields));
            }
        }

        Ok(csv_data)
    }

    /// Parse CSV from file
    fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<CsvData, ProcessorError> {
        let file = File::open(&path)
            .map_err(|e| ProcessorError::IoError(format!("Cannot open file: {}", e)))?;
        
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut csv_data = if self.has_headers {
            if let Some(Ok(first_line)) = lines.next() {
                let headers = self.parse_line(&first_line);
                CsvData::with_headers(headers)
            } else {
                return Ok(CsvData::new());
            }
        } else {
            CsvData::new()
        };

        for line_result in lines {
            if let Ok(line) = line_result {
                if !line.trim().is_empty() {
                    let fields = self.parse_line(&line);
                    csv_data.add_row(CsvRow::new(fields));
                }
            }
        }

        Ok(csv_data)
    }

    fn parse_line(&self, line: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                }
                c if c == self.delimiter && !in_quotes => {
                    fields.push(current_field.trim().to_string());
                    current_field.clear();
                }
                _ => {
                    current_field.push(ch);
                }
            }
        }

        fields.push(current_field.trim().to_string());
        fields
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

#[derive(Debug)]
struct Statistics {
    count: usize,
    sum: f64,
    mean: f64,
    median: f64,
    std_dev: f64,
    min: f64,
    max: f64,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Statistics:")?;
        writeln!(f, "  Count:    {}", self.count)?;
        writeln!(f, "  Sum:      {:.2}", self.sum)?;
        writeln!(f, "  Mean:     {:.2}", self.mean)?;
        writeln!(f, "  Median:   {:.2}", self.median)?;
        writeln!(f, "  Std Dev:  {:.2}", self.std_dev)?;
        writeln!(f, "  Min:      {:.2}", self.min)?;
        writeln!(f, "  Max:      {:.2}", self.max)?;
        Ok(())
    }
}

struct StatisticsCalculator;

impl StatisticsCalculator {
    /// Calculate statistics for a numeric column
    fn calculate(values: &[f64]) -> Result<Statistics, ProcessorError> {
        if values.is_empty() {
            return Err(ProcessorError::ValidationError("No values to calculate".to_string()));
        }

        let count = values.len();
        let sum: f64 = values.iter().sum();
        let mean = sum / count as f64;

        // Calculate median
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if count % 2 == 0 {
            (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
        } else {
            sorted[count / 2]
        };

        // Calculate standard deviation
        let variance: f64 = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();

        let min = sorted[0];
        let max = sorted[count - 1];

        Ok(Statistics {
            count,
            sum,
            mean,
            median,
            std_dev,
            min,
            max,
        })
    }

    /// Parse string values to floats
    fn parse_numeric_column(values: &[&String]) -> Vec<f64> {
        values
            .iter()
            .filter_map(|v| v.parse::<f64>().ok())
            .collect()
    }
}

// ============================================================================
// REPORT GENERATOR
// ============================================================================

struct ReportGenerator;

impl ReportGenerator {
    /// Generate text report
    fn generate_text_report(csv_data: &CsvData) -> String {
        let mut report = String::new();
        
        report.push_str("=== CSV DATA REPORT ===\n\n");
        report.push_str(&format!("Total Rows: {}\n", csv_data.row_count()));
        report.push_str(&format!("Total Columns: {}\n\n", csv_data.column_count()));

        report.push_str("Columns:\n");
        for (i, header) in csv_data.headers.iter().enumerate() {
            report.push_str(&format!("  {}. {}\n", i + 1, header));
        }

        report.push_str("\n=== DATA PREVIEW (First 5 rows) ===\n\n");
        
        // Print headers
        report.push_str(&csv_data.headers.join(" | "));
        report.push_str("\n");
        report.push_str(&"-".repeat(csv_data.headers.join(" | ").len()));
        report.push_str("\n");

        // Print first 5 rows
        for (i, row) in csv_data.rows.iter().take(5).enumerate() {
            report.push_str(&row.fields.join(" | "));
            report.push_str("\n");
        }

        report
    }

    /// Generate statistics report for numeric columns
    fn generate_statistics_report(csv_data: &CsvData) -> String {
        let mut report = String::new();
        
        report.push_str("\n=== STATISTICS REPORT ===\n\n");

        for (i, header) in csv_data.headers.iter().enumerate() {
            let column = csv_data.get_column_by_index(i);
            let numeric_values = StatisticsCalculator::parse_numeric_column(&column);
            
            if !numeric_values.is_empty() {
                report.push_str(&format!("Column: {}\n", header));
                if let Ok(stats) = StatisticsCalculator::calculate(&numeric_values) {
                    report.push_str(&format!("{}\n", stats));
                }
            }
        }

        report
    }

    /// Generate HTML report
    fn generate_html_report(csv_data: &CsvData) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str("  <title>CSV Data Report</title>\n");
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("    table { border-collapse: collapse; width: 100%; margin: 20px 0; }\n");
        html.push_str("    th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("    th { background-color: #4CAF50; color: white; }\n");
        html.push_str("    tr:nth-child(even) { background-color: #f2f2f2; }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n<body>\n");
        
        html.push_str("  <h1>CSV Data Report</h1>\n");
        html.push_str(&format!("  <p>Total Rows: {}</p>\n", csv_data.row_count()));
        html.push_str(&format!("  <p>Total Columns: {}</p>\n", csv_data.column_count()));
        
        html.push_str("  <h2>Data Preview</h2>\n");
        html.push_str("  <table>\n    <tr>\n");
        
        // Headers
        for header in &csv_data.headers {
            html.push_str(&format!("      <th>{}</th>\n", header));
        }
        html.push_str("    </tr>\n");
        
        // Rows (first 10)
        for row in csv_data.rows.iter().take(10) {
            html.push_str("    <tr>\n");
            for field in &row.fields {
                html.push_str(&format!("      <td>{}</td>\n", field));
            }
            html.push_str("    </tr>\n");
        }
        
        html.push_str("  </table>\n");
        html.push_str("</body>\n</html>");
        
        html
    }

    /// Save report to file
    fn save_report<P: AsRef<Path>>(path: P, content: &str) -> Result<(), ProcessorError> {
        let mut file = File::create(path)
            .map_err(|e| ProcessorError::IoError(format!("Cannot create file: {}", e)))?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| ProcessorError::IoError(format!("Cannot write to file: {}", e)))?;
        
        Ok(())
    }
}

// ============================================================================
// DEMO
// ============================================================================

fn create_sample_csv() -> String {
    r#"Name,Age,Salary,Department
Alice,28,75000,Engineering
Bob,35,85000,Engineering
Charlie,42,95000,Management
David,31,70000,Sales
Eve,29,72000,Sales
Frank,38,88000,Engineering
Grace,33,78000,Marketing
Henry,27,65000,Marketing
Ivy,40,92000,Management
Jack,36,80000,Sales"#
        .to_string()
}

fn main() {
    println!("=== CSV File Processor Demo ===\n");

    // Create sample CSV data
    let csv_content = create_sample_csv();
    println!("Sample CSV Content:");
    println!("{}\n", csv_content);

    // Parse CSV
    println!("1. Parsing CSV...");
    let parser = CsvParser::new();
    match parser.parse_string(&csv_content) {
        Ok(csv_data) => {
            println!("✓ Parsed {} rows with {} columns\n", csv_data.row_count(), csv_data.column_count());

            // Generate text report
            println!("2. Generating Text Report:");
            let text_report = ReportGenerator::generate_text_report(&csv_data);
            println!("{}", text_report);

            // Calculate statistics
            println!("3. Generating Statistics Report:");
            let stats_report = ReportGenerator::generate_statistics_report(&csv_data);
            println!("{}", stats_report);

            // Analyze specific column
            println!("4. Analyzing Salary Column:");
            if let Some(salary_column) = csv_data.get_column("Salary") {
                let numeric_values = StatisticsCalculator::parse_numeric_column(&salary_column);
                match StatisticsCalculator::calculate(&numeric_values) {
                    Ok(stats) => println!("{}", stats),
                    Err(e) => println!("Error: {}", e),
                }
            }

            // Department-wise analysis
            println!("5. Department-wise Employee Count:");
            let dept_column = csv_data.get_column("Department").unwrap();
            let mut dept_counts: std::collections::HashMap<&String, usize> = std::collections::HashMap::new();
            
            for dept in dept_column {
                *dept_counts.entry(dept).or_insert(0) += 1;
            }
            
            for (dept, count) in dept_counts {
                println!("  {}: {} employees", dept, count);
            }

            // Generate HTML report
            println!("\n6. Generating HTML Report:");
            let html_report = ReportGenerator::generate_html_report(&csv_data);
            
            let html_path = "/tmp/csv_report.html";
            match ReportGenerator::save_report(html_path, &html_report) {
                Ok(_) => println!("✓ HTML report saved to {}", html_path),
                Err(e) => println!("✗ Error saving HTML report: {}", e),
            }

            // Save text report
            let text_path = "/tmp/csv_report.txt";
            let full_report = format!("{}\n{}", text_report, stats_report);
            match ReportGenerator::save_report(text_path, &full_report) {
                Ok(_) => println!("✓ Text report saved to {}", text_path),
                Err(e) => println!("✗ Error saving text report: {}", e),
            }
        }
        Err(e) => {
            println!("✗ Error parsing CSV: {}", e);
        }
    }

    println!("\n=== Demo Complete ===");
}
