pub mod database;

use crate::database::setup_database;
use diesel::prelude::*;
use std::{error::Error, path::PathBuf, sync::Mutex};

pub struct GlobalState {
    pub database_connection: Mutex<SqliteConnection>,
}

impl GlobalState {
    pub fn build(database_location: PathBuf) -> Result<GlobalState, Box<dyn Error>> {
        let connection = setup_database(&database_location)?;

        let state = GlobalState {
            database_connection: Mutex::new(connection),
        };

        Ok(state)
    }
}
