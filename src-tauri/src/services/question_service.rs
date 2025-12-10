use std::sync::Arc;

use crate::services::DatabaseService;
use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

pub struct QuestionService {
    db: Arc<Mutex<DatabaseService>>,
}

impl QuestionService {
    pub fn new(db: Arc<Mutex<DatabaseService>>) -> Self {
        Self { db }
    }

    pub async fn query_questions(&self) -> Result<()> {
        let sql = "SELECT id, title, content FROM questions ORDER BY id DESC";
        let db = self.db.lock().await;
        let conn = db.get_connection().await;

        let mut rows = conn
            .query(sql, [0])
            .await
            .map_err(|e| anyhow!("Query failed: {}", e))?;

        // doc ket qua
        while let Some(row) = rows.next().await? {
            println!("id = {:#?}", row.get_value(0)?);
            println!("name = {:#?}", row.get_value(1)?);
            println!("age = {:#?}", row.get_value(2)?);
        }

        Ok(())
    }
}
