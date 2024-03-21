pub mod database;
pub mod projects;
pub mod rfid_tags;

use database::{create_session, setup_database};
use diesel::prelude::*;
use projects::{Project, Theme};
use rfid_tags::run_instance;
use serde::Serialize;
use std::{error::Error, path::PathBuf, process::Child, sync::Mutex};

pub struct CurrentSession {
    pub session_id: i32,
    pub theme: Theme,
}

#[derive(Serialize)]
pub struct Tags {
    id: String,
    strength: i32,
    antenna: i32,
}

pub struct GlobalState {
    pub database_connection: Mutex<SqliteConnection>,
    pub current_project: Mutex<Option<Project>>,
    pub current_session: Mutex<Option<CurrentSession>>,
    pub read_tags: Mutex<Option<Tags>>,
    pub child_handle: Mutex<Option<Child>>,
}

impl GlobalState {
    pub fn build(database_location: PathBuf) -> Result<GlobalState, Box<dyn Error>> {
        let connection = setup_database(&database_location)?;

        let state = GlobalState {
            database_connection: Mutex::new(connection),
            current_project: Mutex::new(None),
            current_session: Mutex::new(None),
            read_tags: Mutex::new(None),
            child_handle: Mutex::new(None),
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

    pub fn toggle_reading(&self) -> Result<bool, String> {
        let mut lock = self.child_handle.lock().unwrap();
        let status = match &mut *lock {
            Some(child) => {
                child.kill().unwrap();
                *lock = None;
                false
            }
            None => {
                *lock = Some(run_instance()?);
                true
            }
        };
        Ok(status)
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
