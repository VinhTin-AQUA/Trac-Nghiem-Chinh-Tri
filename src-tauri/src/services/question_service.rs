use std::sync::Arc;

use crate::{models::{AddQuestion, Question}, services::DatabaseService};
use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

pub struct QuestionService {
    db: Arc<Mutex<DatabaseService>>,
}

impl QuestionService {
    pub fn new(db: Arc<Mutex<DatabaseService>>) -> Self {
        Self { db }
    }

    pub async fn add_question(&self, new_question: AddQuestion) -> Result<i64> {
        let db = self.db.lock().await;
        let conn = db.get_connection().await;

        let t = conn.execute(
            "INSERT INTO questions (content) VALUES (?1)",
            [new_question.content],
        )
        .await?;

        let last_insert_rowid = conn.last_insert_rowid();

        Ok(last_insert_rowid)
    }

    pub async fn get_all_questions(&self) -> Result<Vec<Question>> {
        let sql = "SELECT id, content FROM questions";
        let db = self.db.lock().await;
        let conn = db.get_connection().await;

        let mut rows = conn
            .query(sql, [0])
            .await
            .map_err(|e| anyhow!("Query failed: {}", e))?;

        let mut questions: Vec<Question> = Vec::new();

        // doc ket qua
        while let Some(row) = rows.next().await? {
            questions.push(Question {
                id: row.get::<i64>(0).unwrap_or(0),
                content: row.get::<String>(1).unwrap_or("".to_string()),
            });

        }

        Ok(questions)
    }
}
