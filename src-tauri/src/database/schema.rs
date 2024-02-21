// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Integer,
        created_at -> Timestamp,
        project_key -> Text,
        theme_key -> Text,
    }
}
