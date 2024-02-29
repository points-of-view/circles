// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use erasmus::{database::create_session, GlobalState};
use std::fs;
use tauri::Manager;

#[tauri::command]
fn start_session(state: tauri::State<GlobalState>, project_key: String, theme_key: String) -> i32 {
    let mut connection = state.database_connection.lock().unwrap();

    let session = create_session(&mut *connection, &project_key, &theme_key);
    session.id
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("Error while getting `app_data_dir`");

            // Make sure the data_dir exists
            fs::create_dir_all(&data_dir)?;

            data_dir.push("erasmus_db.sqlite");
            let state = GlobalState::build(data_dir)?;

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
