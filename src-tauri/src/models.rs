use diesel::prelude::*;
use time;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: i32,
    pub current_step: i32,
    pub created_at: time::PrimitiveDateTime,
}
