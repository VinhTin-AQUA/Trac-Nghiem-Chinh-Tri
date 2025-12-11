use std::sync::Arc;
use crate::{app_state::AppState, models::AddQuestion};
use tauri::{command, State};

#[command]
pub async fn add_question(
    state: State<'_, Arc<AppState>>,
    new_question: AddQuestion,
) -> Result<i64, String> {
    let question_service = &state.question_service;

    let r = question_service
        .lock()
        .await
        .add_question(new_question)
        .await
        .map_err(|e| e.to_string());

    r
}
