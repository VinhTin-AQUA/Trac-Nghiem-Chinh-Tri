use crate::{
    app_state::AppState,
    models::{AddQuestion, Question},
};
use std::sync::Arc;
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

#[command]
pub async fn get_all_questions(state: State<'_, Arc<AppState>>) -> Result<Vec<Question>, String> {
    let question_service = &state.question_service;

    let r = question_service
        .lock()
        .await
        .get_all_questions()
        .await
        .map_err(|e| e.to_string());

    r
}
