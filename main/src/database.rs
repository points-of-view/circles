pub mod models;
pub mod schema;

use crate::{
    database::models::{Answer, Session, Step},
    tags::TagsMap,
};
use diesel::{prelude::*, sqlite::Sqlite};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{error::Error, path};

// NOTE: This path is relative to our root, and not this file.
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn setup_database(location: &path::PathBuf) -> Result<SqliteConnection, Box<dyn Error>> {
    let mut connection = establish_connection(&location);
    run_migrations(&mut connection)
        .unwrap_or_else(|err| panic!("Could not run migrations, due to {}", err));

    Ok(connection)
}

pub fn create_session(
    connection: &mut SqliteConnection,
    project_key: &str,
    theme_key: &str,
) -> Session {
    use crate::database::schema::sessions;

    diesel::insert_into(sessions::table)
        .values((
            sessions::project_key.eq(project_key),
            sessions::theme_key.eq(theme_key),
        ))
        .returning(Session::as_returning())
        .get_result(connection)
        .expect("Error saving new session")
}

pub fn create_step(
    connection: &mut SqliteConnection,
    session_id: &i32,
    question_key: &str,
) -> Step {
    use crate::database::schema::steps;

    diesel::insert_into(steps::table)
        .values((
            steps::session_id.eq(session_id),
            steps::question_key.eq(question_key),
        ))
        .returning(Step::as_returning())
        .get_result(connection)
        .expect("Error saving step")
}

pub fn create_answer(
    connection: &mut SqliteConnection,
    step_id: &i32,
    option_key: &str,
    token_key: &str,
) -> Answer {
    use crate::database::schema::answers;

    diesel::insert_into(answers::table)
        .values((
            answers::step_id.eq(step_id),
            answers::option_key.eq(option_key),
            answers::token_key.eq(token_key),
        ))
        .returning(Answer::as_returning())
        .get_result(connection)
        .expect("Error saving answer")
}

pub fn save_step_results(
    connection: &mut SqliteConnection,
    session_id: &i32,
    current_step: &str,
    tags_map: TagsMap,
) -> Result<(Step, usize), String> {
    use crate::database::schema::answers;

    let step = create_step(connection, session_id, current_step);
    let records: Vec<_> = tags_map
        .values()
        .map(|tag| {
            (
                answers::step_id.eq(step.id),
                answers::option_key.eq(tag.antenna.to_string()),
                answers::token_key.eq(&tag.id),
            )
        })
        .collect();

    match diesel::insert_into(answers::table)
        .values(records)
        .execute(connection)
    {
        Ok(answers) => Ok((step, answers)),
        Err(err) => Err(err.to_string()),
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

fn establish_connection(location: &path::PathBuf) -> SqliteConnection {
    SqliteConnection::establish(location.to_str().unwrap())
        .unwrap_or_else(|_| panic!("Error connecting to {:?}", location))
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
            let session = create_session(conn, "testProject", "eco");

            // Every test should create a new in-memory DB, so this id is always 1
            assert_eq!(session.id, 1);

            Ok(())
        })
    }

    #[test]
    fn can_create_step() {
        let mut connection = test_db();

        connection.test_transaction::<_, Error, _>(|conn| {
            let session = create_session(conn, "testProject", "eco");

            let step = create_step(conn, &session.id, "my-question");

            // Every test should create a new in-memory DB, so this id is always 1
            assert_eq!(step.id, 1);

            Ok(())
        })
    }

    #[test]
    fn can_create_answers() {
        let mut connection = test_db();

        connection.test_transaction::<_, Error, _>(|conn| {
            let session = create_session(conn, "testProject", "eco");
            let step = create_step(conn, &session.id, "my-question");
            let answer = create_answer(conn, &step.id, "option-1", "abc123");

            // Every test should create a new in-memory DB, so this id is always 1
            assert_eq!(answer.id, 1);

            Ok(())
        })
    }

    #[test]
    fn can_create_step_and_answers() {
        let mut connection = test_db();

        connection.test_transaction::<_, Error, _>(|conn| {
            let session = create_session(conn, "testProject", "eco");

            let map = TagsMap::random(10);
            let expected_len = &map.values().len();
            let result = save_step_results(conn, &session.id, "my-question", map);

            assert!(result.is_ok());

            let (step, answers_count) = result.unwrap();
            // Every test should create a new in-memory DB, so this id is always 1
            assert_eq!(step.id, 1);
            assert_eq!(step.question_key, "my-question");
            assert_eq!(&answers_count, expected_len);

            Ok(())
        })
    }
}
