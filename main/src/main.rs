// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use circles::{error::CirclesError, export::export_project_data, projects::Project, GlobalState};
use std::fs;
use tauri::Manager;

#[tauri::command]
async fn get_projects() -> Vec<Project> {
    Project::build_all()
}

#[tauri::command]
async fn import_project(filepath: String) -> Result<(), String> {
    // NOTE: We just print the contents of the file to show that we can use it.
    // The actual implementation will follow later
    let contents = fs::read_to_string(&filepath).expect("Should have been able to read the file");
    println!("Content of {filepath}:\n{contents}");
    Ok(())
}

#[tauri::command]
async fn select_project(
    state: tauri::State<'_, GlobalState>,
    app_handle: tauri::AppHandle,
    project_key: String,
    hostname: String,
) -> Result<(), CirclesError> {
    state.select_project(project_key)?;
    state.start_reading(hostname, app_handle)?;
    Ok(())
}

#[tauri::command]
fn start_session(state: tauri::State<GlobalState>, theme_key: String) -> Result<i32, String> {
    state.start_session(theme_key)
}

#[tauri::command]
fn reset_tags_map(state: tauri::State<GlobalState>) {
    state.reset_tags_map()
}

#[tauri::command]
async fn save_step_results(
    state: tauri::State<'_, GlobalState>,
    current_step: String,
) -> Result<(), String> {
    state.save_step_results(current_step)
}

#[tauri::command]
fn close_connection(state: tauri::State<GlobalState>) -> () {
    state.drop_reader();
}

#[tauri::command]
async fn save_export(
    state: tauri::State<'_, GlobalState>,
    filepath: String,
    project_key: String,
) -> Result<(), CirclesError> {
    // NOTE: This allows any arbitrary project_key, but will simply not find results if the project key does not exists
    // Once we move projects to the database, we'll solve this in a more fundamental way
    let mut connection = state.database_connection.lock().unwrap();
    export_project_data(&mut *connection, filepath, project_key)?;
    Ok(())
}

fn main() {
    let app = tauri::Builder::default()
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
            close_connection,
            get_projects,
            import_project,
            reset_tags_map,
            save_export,
            save_step_results,
            select_project,
            start_session,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // Make sure we fully drop the reader when exiting
    app.run(|app_handle, event| match event {
        tauri::RunEvent::Exit { .. } => {
            let state = app_handle.state::<GlobalState>();
            state.drop_reader();
        }
        _ => {}
    })
}
