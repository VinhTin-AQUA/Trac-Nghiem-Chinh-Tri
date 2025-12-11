use std::sync::Arc;

use crate::{models::AddAnswer, services::DatabaseService};
use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

pub struct AnswerService {
    db: Arc<Mutex<DatabaseService>>,
}

impl AnswerService {
    pub fn new(db: Arc<Mutex<DatabaseService>>) -> Self {
        Self { db }
    }

    pub async fn add_answers(&self, new_answers: Vec<AddAnswer>) -> Result<()> {
        let mut db = self.db.lock().await;
        let conn = db.get_connection_mut().await;
        let tx = conn.transaction().await?;

        for ans in new_answers {
            let is_correct_str = if ans.is_correct { "1" } else { "0" };

            tx.execute(
                "INSERT INTO answers (content,is_correct,question_id) VALUES (?1)",
                [
                    ans.content,
                    is_correct_str.to_string(),
                    ans.question_id.to_string(),
                ],
            )
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn query_answers(&self) -> Result<()> {
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
