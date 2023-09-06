-- Your SQL goes here
create table subject_runs (
    id BLOB PRIMARY KEY,
    subject_id BLOB NOT NULL,
    `datetime` TEXT NOT NULL
) STRICT;