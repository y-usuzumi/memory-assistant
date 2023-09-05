// @generated automatically by Diesel CLI.

diesel::table! {
    subject_descs (id) {
        id -> Binary,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        max_runs -> Nullable<Integer>,
    }
}
