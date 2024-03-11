use diesel::prelude::*;
use time;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: i32,
    pub created_at: time::PrimitiveDateTime,
    pub project_key: String,
    pub theme_key: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::steps)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Step {
    pub id: i32,
    pub created_at: time::PrimitiveDateTime,
    pub session_id: i32,
    pub question_key: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::answers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Answer {
    pub id: i32,
    pub step_id: i32,
    pub option_key: String,
    pub token_key: String,
}
