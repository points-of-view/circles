// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use erasmus::{reader::spawn_reader, GlobalState};
use std::fs;
use tauri::{api::process::CommandEvent, Manager};

#[tauri::command]
fn select_project(
    state: tauri::State<GlobalState>,
    app_handle: tauri::AppHandle,
    project_key: String,
) -> Result<(), String> {
    state.select_project(project_key)?;
    state.start_reading(app_handle)
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

            let resource_dir = app
                .path_resolver()
                .resource_dir()
                .expect("Error while getting `resource_dir`");

            println!(
                "data_dir: {:?}, resource dir: {:?}",
                &data_dir, resource_dir
            );

            // Make sure the data_dir exists
            fs::create_dir_all(&data_dir)?;

            let (mut rx, _child) = spawn_reader(app.handle());
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stderr(line) => println!("stderr: {}", line),
                        CommandEvent::Stdout(line) => println!("stdout: {}", line),
                        CommandEvent::Error(ee) => println!("error: {}", ee),
                        CommandEvent::Terminated(_) => println!("Terminated!"),
                        _ => todo!(),
                    }
                }
            });

            data_dir.push("erasmus_db.sqlite");
            let state = GlobalState::build(data_dir)?;

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_project, start_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
