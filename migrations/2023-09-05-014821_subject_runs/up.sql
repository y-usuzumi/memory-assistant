-- Your SQL goes here
create table subject_runs (
    id BLOB PRIMARY KEY,
    subject_id BLOB NOT NULL,
    `time` INTEGER NOT NULL
) STRICT;