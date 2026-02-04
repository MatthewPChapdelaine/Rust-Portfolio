use sqlx::{SqlitePool, Row};
use crate::models::ChatMessage;
use std::error::Error;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                room TEXT NOT NULL,
                username TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                is_private BOOLEAN NOT NULL DEFAULT 0
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS rooms (
                name TEXT PRIMARY KEY,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_message(&self, message: &ChatMessage) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO messages (id, room, username, content, timestamp, is_private) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&message.id)
        .bind(&message.room)
        .bind(&message.username)
        .bind(&message.content)
        .bind(&message.timestamp)
        .bind(message.is_private)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_room_messages(&self, room: &str, limit: i32) -> Result<Vec<ChatMessage>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT * FROM messages WHERE room = ? AND is_private = 0 ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(room)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(ChatMessage {
                id: row.get("id"),
                room: row.get("room"),
                username: row.get("username"),
                content: row.get("content"),
                timestamp: row.get("timestamp"),
                is_private: row.get("is_private"),
            });
        }

        messages.reverse();
        Ok(messages)
    }

    pub async fn create_room(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("INSERT OR IGNORE INTO rooms (name, created_at) VALUES (?, ?)")
            .bind(name)
            .bind(now)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_all_rooms(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let rows = sqlx::query("SELECT name FROM rooms ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.iter().map(|row| row.get("name")).collect())
    }
}
