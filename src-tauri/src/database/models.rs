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
