pub mod database;
pub mod error;
pub mod export;
pub mod projects;
pub mod reader;
pub mod tags;

use database::{create_session, save_step_results, setup_database};
use diesel::prelude::*;
use error::{GeneralError, GeneralErrorKind};
use projects::{Project, Theme};
use reader::{LLRPReader, MockReader, Reader, ReaderError, ReaderProtocol};
use std::{env, path::PathBuf};
use tags::TagsMap;
use tauri::AppHandle;

#[derive(Clone)]
pub struct CurrentSession {
    pub session_id: i32,
    pub theme: Theme,
}

pub struct GlobalState {
    pub database_connection: std::sync::Mutex<SqliteConnection>,
    pub current_project: std::sync::Mutex<Option<Project>>,
    pub current_session: std::sync::Mutex<Option<CurrentSession>>,
    pub reader: std::sync::Mutex<Option<Reader>>,
}

impl GlobalState {
    pub fn build(database_location: PathBuf) -> Result<GlobalState, Box<dyn std::error::Error>> {
        let connection = setup_database(&database_location)?;

        let state = GlobalState {
            database_connection: std::sync::Mutex::new(connection),
            current_project: std::sync::Mutex::new(None),
            current_session: std::sync::Mutex::new(None),
            reader: std::sync::Mutex::new(None),
        };

        Ok(state)
    }

    pub fn select_project(&self, project_key: String) -> Result<(), GeneralError> {
        match Project::build_all()
            .iter()
            .find(|&project| project.key == project_key)
        {
            Some(project) => {
                let mut lock = self.current_project.lock().unwrap();
                *lock = Some(project.clone());
                Ok(())
            }
            None => Err(GeneralError {
                kind: GeneralErrorKind::IncorrectProject(project_key),
                message: String::new(),
            }),
        }
    }

    pub fn start_session(&self, theme_key: String) -> Result<i32, String> {
        let mut project = self.current_project.lock().unwrap();
        let project = match &mut *project {
            Some(p) => p,
            None => return Err("Please select a project first".to_string()),
        };

        let theme = match project.themes.iter().find(|&theme| theme.key == theme_key) {
            Some(t) => t.clone(),
            None => return Err("Could not find theme with this code!".to_string()),
        };

        let mut connection = self.database_connection.lock().unwrap();
        let mut current_session = self.current_session.lock().unwrap();

        let session = create_session(&mut *connection, &project.key, &theme_key);
        *current_session = Some(CurrentSession {
            session_id: session.id,
            theme: theme,
        })
        .into();

        Ok(session.id)
    }

    pub fn start_reading<R: tauri::Runtime>(
        &self,
        hostname: String,
        app_handle: AppHandle<R>,
    ) -> Result<(), ReaderError> {
        let mut lock = self.reader.lock().unwrap();
        // If the user refreshes we *might* already have a reader that is connected
        if let Some(reader) = lock.take() {
            drop(reader);
        }

        let mut reader = match env::var("MOCK_RFID_READER") {
            Ok(_) => Reader::MockReader(MockReader::new(hostname)?),
            Err(_) => Reader::LLRPReader(LLRPReader::new(hostname)?),
        };

        reader.start_reading(app_handle)?;
        *lock = Some(reader);
        Ok(())
    }

    pub fn save_step_results(&self, current_step: String, tags_map: TagsMap) -> Result<(), String> {
        let mut connection = self.database_connection.lock().unwrap();
        let current_session = self.current_session.lock().unwrap();

        if current_session.is_some() {
            save_step_results(
                &mut *connection,
                &current_session.as_ref().unwrap().session_id,
                &current_step,
                tags_map,
            )?;
            Ok(())
        } else {
            Err(String::from("No current session"))
        }
    }

    pub fn stop_reading(&self, await_confirmation: bool) -> Result<(), ReaderError> {
        let mut lock = self.reader.lock().unwrap();
        if let Some(reader) = &mut *lock {
            reader.stop_reading(await_confirmation)
        } else {
            Ok(())
        }
    }

    pub fn drop_reader(&self) {
        let mut lock = self.reader.lock().unwrap();
        lock.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_okay_if_project_exists() {
        let state = GlobalState::build(":memory:".into()).unwrap();

        assert!(state.select_project("test".to_string()).is_ok())
    }

    #[test]
    fn should_err_if_project_does_not_exist() {
        let state = GlobalState::build(":memory:".into()).unwrap();

        assert!(state.select_project("faulty-project".to_string()).is_err())
    }

    #[test]
    fn should_return_session_id_if_theme_exists() {
        let state = GlobalState::build(":memory:".into()).unwrap();
        state.select_project("test".to_string()).unwrap();

        assert!(state.start_session("theme-one".to_string()).is_ok())
    }

    #[test]
    fn should_err_if_theme_does_not_exist() {
        let state = GlobalState::build(":memory:".into()).unwrap();
        state.select_project("test".to_string()).unwrap();

        assert!(state.start_session("theme-zero".to_string()).is_err())
    }

    #[test]
    fn should_err_if_starting_session_before_project() {
        let state = GlobalState::build(":memory:".into()).unwrap();

        assert!(state.start_session("theme-one".to_string()).is_err())
    }
}
