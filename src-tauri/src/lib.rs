mod commands;
use std::sync::Arc;

use commands::*;

mod services;
use services::*;

mod models;

mod states;
use states::*;

use tauri::Manager;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // block_on để chạy async trong setup
            let db_service =
                tauri::async_runtime::block_on(async { DatabaseService::new().await.unwrap() });
            let db_service = Arc::new(Mutex::new(db_service));
            let question_service = Arc::new(Mutex::new(QuestionService::new(db_service.clone())));

            let state = AppState {
                db_service,
                question_service,
            };

            app.manage(Arc::new(state));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
