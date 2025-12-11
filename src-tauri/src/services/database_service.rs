use crate::constants::{app_folder_constants, database_constants};
use anyhow::{anyhow, Result};
use dirs;
use std::path::PathBuf;
use turso::{Builder, Connection, Row};

pub struct DatabaseService {
    connection: Connection,
}

impl DatabaseService {
    pub async fn new() -> Result<Self> {
        let data_local_dir = dirs::data_local_dir().unwrap_or(PathBuf::from("./db"));
        let db_path = data_local_dir
            .join(app_folder_constants::APP_PACKAGE)
            .join(database_constants::DB_PATH);

        // Pure async, không blocking
        let db = Builder::new_local(db_path.to_string_lossy().to_string().as_str())
            .build()
            .await
            .map_err(|e| anyhow!("Failed to build database: {}", e))?;

        let conn = db
            .connect()
            .map_err(|e| anyhow!("Failed to connect: {}", e))?;

        // Tạo bảng posts
        conn.execute(
            r#"
                    CREATE TABLE IF NOT EXISTS questions (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        content TEXT NOT NULL
                    );
                "#,
            (),
        )
        .await
        .map_err(|e| anyhow!("Failed to create table: {}", e))?;

        // Tạo bảng users
        conn.execute(
            r#"
                    CREATE TABLE IF NOT EXISTS answers (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        question_id INT NOT NULL,
                        content TEXT NOT NULL,
                        is_correct BOOLEAN DEFAULT FALSE
                    );
                "#,
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
