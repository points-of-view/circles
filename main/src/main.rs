// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use circles::GlobalState;
use std::fs;
use tauri::Manager;

#[tauri::command]
fn select_project(
    state: tauri::State<GlobalState>,
    app_handle: tauri::AppHandle,
    project_key: String,
    hostname: String,
) -> Result<(), String> {
    state.select_project(project_key)?;
    if let Err(err) = state.start_reading(hostname, app_handle) {
        Err(err.to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
fn start_session(state: tauri::State<GlobalState>, theme_key: String) -> Result<i32, String> {
    state.start_session(theme_key)
}

#[tauri::command]
fn close_connection(state: tauri::State<GlobalState>) -> () {
    state.drop_reader();
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

            data_dir.push("circles_db.sqlite");

            // Setup global state and channels for communication
            let state = GlobalState::build(data_dir)?;
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            select_project,
            start_session,
            close_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
