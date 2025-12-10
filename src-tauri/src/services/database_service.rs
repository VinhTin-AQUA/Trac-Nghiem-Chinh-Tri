use anyhow::{anyhow, Result};
use turso::{Builder, Connection, Row};

pub struct DatabaseService {
    connection: Connection,
}

impl DatabaseService {
    pub async fn new() -> Result<Self> {
        // Pure async, không blocking
        let db = Builder::new_local("my-db.db")
            .build()
            .await
            .map_err(|e| anyhow!("Failed to build database: {}", e))?;

        let conn = db
            .connect()
            .map_err(|e| anyhow!("Failed to connect: {}", e))?;

        // Tạo bảng posts
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"#,
            (),
        )
        .await
        .map_err(|e| anyhow!("Failed to create table: {}", e))?;

        // Tạo bảng users
        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"#,
            (),
        )
        .await
        .map_err(|e| anyhow!("Failed to create users table: {}", e))?;

        Ok(DatabaseService { connection: conn })
    }

    pub async fn get_connection(&self) -> &Connection {
        &self.connection
    }

    pub async fn query(&self, sql: &str, params: Vec<turso::Value>) -> Result<Vec<Row>> {
        let conn = self.get_connection().await;
        let mut rows = conn
            .query(sql, params)
            .await
            .map_err(|e| anyhow!("Query failed: {}", e))?;

        let mut result = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| anyhow!("Row iteration failed: {}", e))?
        {
            result.push(row);
        }

        Ok(result)
    }

    // 🔵 Helper method để query single row
    pub async fn query_one(&self, sql: &str, params: Vec<turso::Value>) -> Result<Option<Row>> {
        let mut rows = self.query(sql, params).await?;
        Ok(rows.pop())
    }
}
