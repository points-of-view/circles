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
) -> Result<(), String> {
    state.select_project(project_key)?;

    // NOTE: We resolve the resource_path here instead of in the final method
    // This way we don't have to create an AppHandle in testing
    let resource_path = app_handle
        .path_resolver()
        .resource_dir()
        .expect("Error while getting `resource_dir`");
    state.start_reading(resource_path, app_handle)
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

            data_dir.push("circles_db.sqlite");

            // Setup global state and channels for communication
            let state = GlobalState::build(data_dir)?;
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_project, start_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
