use crate::services::{DatabaseService, QuestionService};
use tokio::sync::Mutex;

pub struct AppState {
    pub db_service: Mutex<DatabaseService>,
    pub question_service: Mutex<QuestionService>,
}
