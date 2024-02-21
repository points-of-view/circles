// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Integer,
        token_key -> Text,
        option_key -> Text,
        step_id -> Integer,
    }
}

diesel::table! {
    sessions (id) {
        id -> Integer,
        created_at -> Timestamp,
        project_key -> Text,
        theme_key -> Text,
    }
}

diesel::table! {
    steps (id) {
        id -> Integer,
        created_at -> Timestamp,
        question_key -> Text,
        session_id -> Integer,
    }
}

diesel::joinable!(answers -> steps (step_id));
diesel::joinable!(steps -> sessions (session_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    sessions,
    steps,
);
