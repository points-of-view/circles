// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use erasmus::GlobalState;
use std::fs;
use tauri::Manager;

#[tauri::command]
fn select_project(
    state: tauri::State<GlobalState>,
    app_handle: tauri::AppHandle,
    project_key: String,
) -> Result<(), String> {
    state.select_project(project_key)?;
    state.start_reading(&app_handle)
}

#[tauri::command]
fn start_session(state: tauri::State<GlobalState>, theme_key: String) -> Result<i32, String> {
    state.start_session(theme_key)
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
        .invoke_handler(tauri::generate_handler![select_project, start_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
