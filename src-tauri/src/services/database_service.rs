use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use turso::{Builder, Connection, Row};

pub struct DatabaseService {
    connection: Connection,
}

impl DatabaseService {
    async fn new() -> Result<Self> {
        // 🟢 Pure async, không blocking
        let db = Builder::new_local("my-DatabaseService.db")
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

    // 🔵 Public method để truy cập connection
    pub async fn get_connection(&self) -> &Connection {
        &self.connection
    }

    // 🔵 Helper method để execute query
    // pub async fn execute(&self, sql: &str, params: Vec<turso::Value>) -> Result<usize> {
    //     let conn = self.get_connection().await;
    //     conn.execute(sql, params)
    //         .await
    //         .map_err(|e| anyhow!("Execute failed: {}", e))
    // }

    // 🔵 Helper method để query rows
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

// 🔵 Singleton instance - CẦN XỬ LÝ LỖI
static DB_INSTANCE: Lazy<Arc<DatabaseService>> = Lazy::new(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(DatabaseService::new()).unwrap();
    Arc::new(db)
});

pub fn db_instance() -> Arc<DatabaseService> {
    Arc::clone(&DB_INSTANCE)
}
