pub mod database;
pub mod projects;
pub mod reader;
pub mod tags;

use database::{create_session, setup_database};
use diesel::prelude::*;
use projects::{Project, Theme};
use reader::{command::spawn_reader, handle_reader_event};
use std::{error::Error, path::PathBuf, sync::Mutex};
use tauri::{
    api::process::CommandChild,
    async_runtime::{spawn, JoinHandle},
};

pub struct CurrentSession {
    pub session_id: i32,
    pub theme: Theme,
}

pub struct GlobalState {
    pub database_connection: Mutex<SqliteConnection>,
    pub current_project: Mutex<Option<Project>>,
    pub current_session: Mutex<Option<CurrentSession>>,
    pub reader_handle: Mutex<Option<(CommandChild, JoinHandle<()>)>>,
}

impl GlobalState {
    pub fn build(database_location: PathBuf) -> Result<GlobalState, Box<dyn Error>> {
        let connection = setup_database(&database_location)?;

        let state = GlobalState {
            database_connection: Mutex::new(connection),
            current_project: Mutex::new(None),
            current_session: Mutex::new(None),
            reader_handle: Mutex::new(None),
        };

        Ok(state)
    }

    pub fn select_project(&self, project_key: String) -> Result<(), String> {
        match Project::build_all()
            .iter()
            .find(|&project| project.key == project_key)
        {
            Some(project) => {
                let mut lock = self.current_project.lock().unwrap();
                *lock = Some(project.clone());
                Ok(())
            }
            None => Err("Project could not be found!".to_owned()),
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

    pub fn start_reading(&self, resource_path: PathBuf) -> Result<(), String> {
        let mut lock = self.reader_handle.lock().unwrap();
        if let Some((child, handle)) = lock.take() {
            // We first abort reading data
            handle.abort();
            // And then kill the child
            child.kill().unwrap();
        }

        let (mut rx, child) = spawn_reader(resource_path);
        let handle = spawn(async move {
            while let Some(event) = rx.recv().await {
                handle_reader_event(event)
            }
        });
        *lock = Some((child, handle));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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

    #[test]
    fn should_return_ok_when_starting_reader() {
        // Make sure we don't call the actual reader code
        env::set_var("MOCK_RFID_READER", "1");
        let state = GlobalState::build(":memory:".into()).unwrap();

        assert!(state.start_reading("/".into()).is_ok());
        let lock = state.reader_handle.lock().unwrap();
        assert!(lock.is_some());
    }
}
