// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Integer,
        created_at -> Timestamp,
        current_step -> Integer,
    }
}
