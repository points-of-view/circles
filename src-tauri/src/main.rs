// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use erasmus::{create_session, GlobalState};

#[tauri::command]
fn start_session(state: tauri::State<GlobalState>) -> i32 {
    let mut connection = state.connection.lock().unwrap();

    let session = create_session(&mut *connection);
    session.id
}

fn main() {
    let state = GlobalState::build();

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![start_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
