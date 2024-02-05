pub mod models;
pub mod schema;

use crate::models::Session;
use diesel::{prelude::*, sqlite::Sqlite};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{env, error::Error, sync::Mutex};

// NOTE: This path is relative to our root, and not this file.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct GlobalState {
    pub connection: Mutex<SqliteConnection>,
}

impl GlobalState {
    pub fn build() -> GlobalState {
        let mut connection = establish_connection();
        run_migrations(&mut connection)
            .unwrap_or_else(|err| panic!("Could not run migrations, due to {}", err));

        GlobalState {
            connection: Mutex::new(connection),
        }
    }
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_session(connection: &mut SqliteConnection) -> Session {
    use crate::schema::sessions;

    diesel::insert_into(sessions::table)
        .default_values()
        .returning(Session::as_returning())
        .get_result(connection)
        .expect("Error saving new session")
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;

    fn test_db() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:")
            .unwrap_or_else(|_| panic!("Could not create in memory DB"));

        run_migrations(&mut conn)
            .unwrap_or_else(|err| panic!("Could not run migrations, due to {}", err));
        conn
    }

    #[test]
    fn can_create_session() {
        let mut connection = test_db();

        connection.test_transaction::<_, Error, _>(|conn| {
            let session = create_session(conn);

            // Every test should create a new in-memory DB, so this id is always 1
            assert_eq!(session.id, 1);

            Ok(())
        })
    }
}
