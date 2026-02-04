use sqlx::{SqlitePool, Row};
use crate::models::{User, Post, Comment};
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
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                is_admin BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS posts (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                slug TEXT UNIQUE NOT NULL,
                content TEXT NOT NULL,
                summary TEXT NOT NULL,
                author_id TEXT NOT NULL,
                published BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (author_id) REFERENCES users(id)
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                post_id TEXT NOT NULL,
                author_name TEXT NOT NULL,
                author_email TEXT NOT NULL,
                content TEXT NOT NULL,
                approved BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                FOREIGN KEY (post_id) REFERENCES posts(id)
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // User operations
    pub async fn create_user(&self, user: &User) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, is_admin, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.is_admin)
        .bind(&user.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn Error>> {
        let row = sqlx::query("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                is_admin: row.get("is_admin"),
                created_at: row.get("created_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        let row = sqlx::query("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                is_admin: row.get("is_admin"),
                created_at: row.get("created_at"),
            }))
        } else {
            Ok(None)
        }
    }

    // Post operations
    pub async fn create_post(&self, post: &Post) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO posts (id, title, slug, content, summary, author_id, published, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&post.id)
        .bind(&post.title)
        .bind(&post.slug)
        .bind(&post.content)
        .bind(&post.summary)
        .bind(&post.author_id)
        .bind(post.published)
        .bind(&post.created_at)
        .bind(&post.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_post_by_slug(&self, slug: &str) -> Result<Option<Post>, Box<dyn Error>> {
        let row = sqlx::query("SELECT * FROM posts WHERE slug = ?")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(Post {
                id: row.get("id"),
                title: row.get("title"),
                slug: row.get("slug"),
                content: row.get("content"),
                summary: row.get("summary"),
                author_id: row.get("author_id"),
                published: row.get("published"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_posts(&self, published_only: bool) -> Result<Vec<Post>, Box<dyn Error>> {
        let query = if published_only {
            "SELECT * FROM posts WHERE published = 1 ORDER BY created_at DESC"
        } else {
            "SELECT * FROM posts ORDER BY created_at DESC"
        };

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let posts = rows
            .iter()
            .map(|row| Post {
                id: row.get("id"),
                title: row.get("title"),
                slug: row.get("slug"),
                content: row.get("content"),
                summary: row.get("summary"),
                author_id: row.get("author_id"),
                published: row.get("published"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(posts)
    }

    pub async fn update_post(&self, post: &Post) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "UPDATE posts SET title = ?, content = ?, summary = ?, published = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.summary)
        .bind(post.published)
        .bind(&post.updated_at)
        .bind(&post.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_post(&self, id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // Comment operations
    pub async fn create_comment(&self, comment: &Comment) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            "INSERT INTO comments (id, post_id, author_name, author_email, content, approved, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&comment.id)
        .bind(&comment.post_id)
        .bind(&comment.author_name)
        .bind(&comment.author_email)
        .bind(&comment.content)
        .bind(comment.approved)
        .bind(&comment.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_comments_by_post(&self, post_id: &str, approved_only: bool) -> Result<Vec<Comment>, Box<dyn Error>> {
        let query = if approved_only {
            "SELECT * FROM comments WHERE post_id = ? AND approved = 1 ORDER BY created_at ASC"
        } else {
            "SELECT * FROM comments WHERE post_id = ? ORDER BY created_at ASC"
        };

        let rows = sqlx::query(query)
            .bind(post_id)
            .fetch_all(&self.pool)
            .await?;

        let comments = rows
            .iter()
            .map(|row| Comment {
                id: row.get("id"),
                post_id: row.get("post_id"),
                author_name: row.get("author_name"),
                author_email: row.get("author_email"),
                content: row.get("content"),
                approved: row.get("approved"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(comments)
    }

    pub async fn approve_comment(&self, id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("UPDATE comments SET approved = 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_comment(&self, id: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query("DELETE FROM comments WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
