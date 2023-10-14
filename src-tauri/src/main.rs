// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::Db;
use klania_state::KlaniaState;
use tauri::Manager;

pub mod db;
pub mod klania_state;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    #[cfg(any(target_os = "linux"))]
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    tauri::Builder::default()
        .manage(KlaniaState::default())
        .setup(|app| {
            let app_handle = app.handle();

            tokio::spawn(async move {
                let app_state: tauri::State<KlaniaState> = app_handle.state();
                
                let db = Db::new("klania.sqlite3".to_string());
                let conn = db.init(&app_handle).await;

                *app_state.db.lock().await = Some(conn);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
