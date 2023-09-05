// @generated automatically by Diesel CLI.

diesel::table! {
    subject_runs (id) {
        id -> Binary,
        subject_id -> Binary,
        time -> Integer,
    }
}

diesel::table! {
    subjects (id) {
        id -> Binary,
        title -> Text,
        description -> Text,
        max_runs -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    subject_runs,
    subjects,
);
