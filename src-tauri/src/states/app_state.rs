use std::sync::Arc;

use crate::services::{DatabaseService, QuestionService};
use tokio::sync::Mutex;

pub struct AppState {
    pub db_service: Arc<Mutex<DatabaseService>>,
    pub question_service: Arc<Mutex<QuestionService>>,
}
