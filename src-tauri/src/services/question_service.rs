use crate::{
    services::{db_instance, DatabaseService},
    states::AppState,
};
use tauri::{Manager, Window};
use tokio::sync::Mutex;

pub struct QuestionService;

impl QuestionService {
    pub async fn create_table(window: &Window) -> turso::Result<()> {
        let db = db_instance(); // lấy Arc<DatabaseService>
        let conn = db.get_connection().await;

        conn.execute(
            r#"CREATE TABLE IF NOT EXISTS students (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                age INTEGER NOT NULL
            )"#,
            (),
        )
        .await?;

        Ok(())
    }

    // pub async fn create(name: &str, age: i32) -> turso::Result<i64> {
    //     let db = DatabaseService::instance().await.lock().await;
    //     let conn = db.connect()?;

    //     let _ = conn
    //         .execute(
    //             "INSERT INTO students (name, age) VALUES (?1, ?2)",
    //             (name, age),
    //         )
    //         .await?;

    //     let mut rows = conn.query("SELECT last_insert_rowid()", ()).await?;
    //     if let Some(row) = rows.next().await? {
    //         let id: i64 = row.get_value(0)?;
    //         Ok(id)
    //     } else {
    //         Ok(0)
    //     }
    // }

    // pub async fn get_all() -> turso::Result<Vec<Question>> {
    //     let db = DatabaseService::instance().await.lock().await;
    //     let conn = db.connect()?;

    //     let mut rows = conn.query("SELECT id, name, age FROM students", ()).await?;
    //     let mut result = vec![];

    //     while let Some(row) = rows.next().await? {
    //         result.push(Question {
    //             id: row.get_value(0)?,
    //             name: row.get_value(1)?,
    //             age: row.get_value(2)?,
    //         });
    //     }

    //     Ok(result)
    // }
}
