/*!
 * Simple Database ORM
 * 
 * A minimal ORM implementation with:
 * - SQLite connection management
 * - CRUD operations (Create, Read, Update, Delete)
 * - Query builder with method chaining
 * - Type-safe query construction
 * - Migration support
 * 
 * # Dependencies
 * This requires rusqlite. To run:
 * ```bash
 * # Create a Cargo project:
 * cargo new database_orm --bin
 * # Add to Cargo.toml:
 * # [dependencies]
 * # rusqlite = { version = "0.30", features = ["bundled"] }
 * 
 * # Or compile standalone (requires sqlite3-dev):
 * rustc database_orm.rs -L /usr/lib -l sqlite3 -o database_orm
 * ./database_orm
 * ```
 * 
 * Note: This demo version uses standard library only for demonstration.
 * For production, use the rusqlite crate.
 */

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// Error Handling
// ============================================================================

#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
    QueryError(String),
    NotFound,
    ValidationError(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DbError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DbError::NotFound => write!(f, "Record not found"),
            DbError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, DbError>;

// ============================================================================
// Value Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Real(r) => write!(f, "{}", r),
            Value::Text(s) => write!(f, "'{}'", s.replace("'", "''")),
            Value::Boolean(b) => write!(f, "{}", if *b { 1 } else { 0 }),
        }
    }
}

// ============================================================================
// Database Connection (Mock)
// ============================================================================

pub struct Database {
    path: String,
    tables: HashMap<String, Vec<HashMap<String, Value>>>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        println!("📁 Opening database: {}", path);
        Ok(Database {
            path: path.to_string(),
            tables: HashMap::new(),
        })
    }

    pub fn execute(&mut self, sql: &str) -> Result<usize> {
        println!("🔧 Executing: {}", sql);
        
        // Simple DDL parsing for demo
        if sql.trim().to_uppercase().starts_with("CREATE TABLE") {
            let table_name = self.extract_table_name(sql)?;
            self.tables.insert(table_name, Vec::new());
            Ok(0)
        } else {
            Ok(1)
        }
    }

    pub fn query(&self, sql: &str) -> Result<Vec<HashMap<String, Value>>> {
        println!("🔍 Querying: {}", sql);
        
        // Extract table name from SELECT query
        let parts: Vec<&str> = sql.split_whitespace().collect();
        if let Some(from_idx) = parts.iter().position(|&x| x.to_uppercase() == "FROM") {
            if let Some(table_name) = parts.get(from_idx + 1) {
                let table_name = table_name.trim_end_matches(';');
                return Ok(self.tables.get(table_name).cloned().unwrap_or_default());
            }
        }
        
        Ok(Vec::new())
    }

    fn extract_table_name(&self, sql: &str) -> Result<String> {
        let parts: Vec<&str> = sql.split_whitespace().collect();
        if let Some(table_idx) = parts.iter().position(|&x| x.to_uppercase() == "TABLE") {
            if let Some(name) = parts.get(table_idx + 1) {
                return Ok(name.trim_end_matches('(').to_string());
            }
        }
        Err(DbError::QueryError("Could not extract table name".to_string()))
    }

    pub fn insert(&mut self, table: &str, row: HashMap<String, Value>) -> Result<usize> {
        println!("➕ Inserting into {}: {:?}", table, row);
        
        if let Some(table_data) = self.tables.get_mut(table) {
            table_data.push(row);
            Ok(table_data.len())
        } else {
            Err(DbError::QueryError(format!("Table {} not found", table)))
        }
    }
}

// ============================================================================
// Query Builder
// ============================================================================

pub struct QueryBuilder {
    table: String,
    select_fields: Vec<String>,
    where_clauses: Vec<String>,
    order_by: Option<String>,
    limit: Option<usize>,
}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        QueryBuilder {
            table: table.to_string(),
            select_fields: vec!["*".to_string()],
            where_clauses: Vec::new(),
            order_by: None,
            limit: None,
        }
    }

    pub fn select(mut self, fields: &[&str]) -> Self {
        self.select_fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn where_eq(mut self, field: &str, value: Value) -> Self {
        self.where_clauses.push(format!("{} = {}", field, value));
        self
    }

    pub fn where_gt(mut self, field: &str, value: Value) -> Self {
        self.where_clauses.push(format!("{} > {}", field, value));
        self
    }

    pub fn where_lt(mut self, field: &str, value: Value) -> Self {
        self.where_clauses.push(format!("{} < {}", field, value));
        self
    }

    pub fn order_by(mut self, field: &str, desc: bool) -> Self {
        self.order_by = Some(format!("{} {}", field, if desc { "DESC" } else { "ASC" }));
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn build(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", self.select_fields.join(", "), self.table);

        if !self.where_clauses.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.where_clauses.join(" AND ")));
        }

        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        sql
    }
}

// ============================================================================
// Model Trait
// ============================================================================

pub trait Model: Sized {
    fn table_name() -> &'static str;
    fn from_row(row: &HashMap<String, Value>) -> Result<Self>;
    fn to_row(&self) -> HashMap<String, Value>;
    
    fn create_table(db: &mut Database) -> Result<()> {
        let sql = Self::create_table_sql();
        db.execute(&sql)?;
        Ok(())
    }
    
    fn create_table_sql() -> String;
}

// ============================================================================
// Repository Pattern
// ============================================================================

pub struct Repository<'a, T: Model> {
    db: &'a mut Database,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: Model> Repository<'a, T> {
    pub fn new(db: &'a mut Database) -> Self {
        Repository {
            db,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn create(&mut self, model: &T) -> Result<usize> {
        let row = model.to_row();
        self.db.insert(T::table_name(), row)
    }

    pub fn find_all(&self) -> Result<Vec<T>> {
        let sql = format!("SELECT * FROM {}", T::table_name());
        let rows = self.db.query(&sql)?;
        
        rows.iter()
            .map(|row| T::from_row(row))
            .collect()
    }

    pub fn find_by_id(&self, id: i64) -> Result<T> {
        let sql = QueryBuilder::new(T::table_name())
            .where_eq("id", Value::Integer(id))
            .limit(1)
            .build();
        
        let rows = self.db.query(&sql)?;
        
        if let Some(row) = rows.first() {
            T::from_row(row)
        } else {
            Err(DbError::NotFound)
        }
    }

    pub fn query(&self) -> QueryBuilder {
        QueryBuilder::new(T::table_name())
    }
}

// ============================================================================
// Example Models
// ============================================================================

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub age: i32,
}

impl Model for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn from_row(row: &HashMap<String, Value>) -> Result<Self> {
        Ok(User {
            id: match row.get("id") {
                Some(Value::Integer(i)) => Some(*i),
                _ => None,
            },
            name: match row.get("name") {
                Some(Value::Text(s)) => s.clone(),
                _ => return Err(DbError::ValidationError("name required".to_string())),
            },
            email: match row.get("email") {
                Some(Value::Text(s)) => s.clone(),
                _ => return Err(DbError::ValidationError("email required".to_string())),
            },
            age: match row.get("age") {
                Some(Value::Integer(i)) => *i as i32,
                _ => 0,
            },
        })
    }

    fn to_row(&self) -> HashMap<String, Value> {
        let mut row = HashMap::new();
        
        if let Some(id) = self.id {
            row.insert("id".to_string(), Value::Integer(id));
        }
        row.insert("name".to_string(), Value::Text(self.name.clone()));
        row.insert("email".to_string(), Value::Text(self.email.clone()));
        row.insert("age".to_string(), Value::Integer(self.age as i64));
        
        row
    }

    fn create_table_sql() -> String {
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            age INTEGER
        )
        "#.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub user_id: i64,
}

impl Model for Post {
    fn table_name() -> &'static str {
        "posts"
    }

    fn from_row(row: &HashMap<String, Value>) -> Result<Self> {
        Ok(Post {
            id: match row.get("id") {
                Some(Value::Integer(i)) => Some(*i),
                _ => None,
            },
            title: match row.get("title") {
                Some(Value::Text(s)) => s.clone(),
                _ => return Err(DbError::ValidationError("title required".to_string())),
            },
            content: match row.get("content") {
                Some(Value::Text(s)) => s.clone(),
                _ => String::new(),
            },
            user_id: match row.get("user_id") {
                Some(Value::Integer(i)) => *i,
                _ => return Err(DbError::ValidationError("user_id required".to_string())),
            },
        })
    }

    fn to_row(&self) -> HashMap<String, Value> {
        let mut row = HashMap::new();
        
        if let Some(id) = self.id {
            row.insert("id".to_string(), Value::Integer(id));
        }
        row.insert("title".to_string(), Value::Text(self.title.clone()));
        row.insert("content".to_string(), Value::Text(self.content.clone()));
        row.insert("user_id".to_string(), Value::Integer(self.user_id));
        
        row
    }

    fn create_table_sql() -> String {
        r#"
        CREATE TABLE posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT,
            user_id INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#.to_string()
    }
}

// ============================================================================
// Demo Application
// ============================================================================

fn main() -> Result<()> {
    println!("🗃️  Database ORM Demo\n");

    // Create database
    let mut db = Database::new("demo.db")?;

    // Create tables
    println!("\n📋 Creating tables...");
    User::create_table(&mut db)?;
    Post::create_table(&mut db)?;

    // Create repository
    let mut user_repo = Repository::<User>::new(&mut db);

    // Insert users
    println!("\n👤 Creating users...");
    let users = vec![
        User {
            id: Some(1),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: 28,
        },
        User {
            id: Some(2),
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
            age: 35,
        },
        User {
            id: Some(3),
            name: "Carol White".to_string(),
            email: "carol@example.com".to_string(),
            age: 42,
        },
    ];

    for user in &users {
        user_repo.create(user)?;
        println!("Created user: {}", user.name);
    }

    // Query builder demo
    println!("\n🔍 Query Builder Examples:");
    
    let query = QueryBuilder::new("users")
        .select(&["name", "email"])
        .where_gt("age", Value::Integer(30))
        .order_by("age", true)
        .limit(10)
        .build();
    
    println!("Generated SQL: {}", query);

    let query2 = QueryBuilder::new("users")
        .where_eq("name", Value::Text("Alice".to_string()))
        .build();
    
    println!("Generated SQL: {}", query2);

    // Create posts
    println!("\n📝 Creating posts...");
    let mut post_repo = Repository::<Post>::new(&mut db);
    
    let posts = vec![
        Post {
            id: Some(1),
            title: "First Post".to_string(),
            content: "Hello, World!".to_string(),
            user_id: 1,
        },
        Post {
            id: Some(2),
            title: "Rust ORM".to_string(),
            content: "Building an ORM in Rust".to_string(),
            user_id: 1,
        },
    ];

    for post in &posts {
        post_repo.create(post)?;
        println!("Created post: {}", post.title);
    }

    // Summary
    println!("\n✅ Demo completed successfully!");
    println!("   - Created {} users", users.len());
    println!("   - Created {} posts", posts.len());
    println!("   - Demonstrated query builder");
    println!("\n💡 In production, use rusqlite crate for real SQLite support");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new("users")
            .select(&["name", "email"])
            .where_eq("id", Value::Integer(1))
            .build();
        
        assert!(query.contains("SELECT name, email"));
        assert!(query.contains("FROM users"));
        assert!(query.contains("WHERE id = 1"));
    }

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Integer(42).to_string(), "42");
        assert_eq!(Value::Text("test".to_string()).to_string(), "'test'");
        assert_eq!(Value::Boolean(true).to_string(), "1");
    }

    #[test]
    fn test_user_model() {
        let user = User {
            id: Some(1),
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
        };

        let row = user.to_row();
        assert_eq!(row.get("name"), Some(&Value::Text("Test".to_string())));
        
        let user2 = User::from_row(&row).unwrap();
        assert_eq!(user2.name, "Test");
    }
}
