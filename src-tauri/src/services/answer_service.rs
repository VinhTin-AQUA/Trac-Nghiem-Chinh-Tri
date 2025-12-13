use std::sync::Arc;

use crate::{
    models::{AddAnswer, Answer},
    services::DatabaseService,
};
use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

pub struct AnswerService {
    db: Arc<Mutex<DatabaseService>>,
}

impl AnswerService {
    pub fn new(db: Arc<Mutex<DatabaseService>>) -> Self {
        Self { db }
    }

    pub async fn add_answers(&self, new_answers: Vec<AddAnswer>) -> Result<bool> {
        let mut db = self.db.lock().await;
        let conn = db.get_connection_mut().await;
        let tx = conn.transaction().await?;

        for ans in new_answers {
            let is_correct_str = if ans.is_correct { "1" } else { "0" };

            tx.execute(
                "INSERT INTO answers (content,is_correct,question_id) VALUES (?1,?2,?3)",
                [
                    ans.content,
                    is_correct_str.to_string(),
                    ans.question_id.to_string(),
                ],
            )
            .await?;
        }
        tx.commit().await?;

        Ok(true)
    }

    pub async fn get_answers_by_question_id(&self, question_id: i64) -> Result<Vec<Answer>> {
        let sql = "SELECT id, content, is_correct, question_id FROM answers where question_id = ?1";
        let db = self.db.lock().await;
        let conn = db.get_connection().await;

        let mut rows = conn
            .query(sql, [question_id])
            .await
            .map_err(|e| anyhow!("Query failed: {}", e))?;

        let mut answrs: Vec<Answer> = Vec::new();

        // doc ket qua
        while let Some(row) = rows.next().await? {
            answrs.push(Answer {
                id: row.get::<i64>(0).unwrap_or(0),
                content: row.get::<String>(1).unwrap_or("".to_string()),
                is_correct: row.get::<bool>(2).unwrap_or(false),
                question_id: row.get::<i64>(3).unwrap_or(0),
            });
        }

        Ok(answrs)
    }

    pub async fn delete_answers_by_question_id(&self, question_id: i64) -> Result<bool> {
        let db = self.db.lock().await;
        let conn = db.get_connection().await;

        conn.execute(
            "DELETE FROM answers WHERE question_id = (?1);",
            [question_id],
        )
        .await?;

        Ok(true)
    }
}
