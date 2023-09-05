-- Your SQL goes here
CREATE TABLE subjects (
    id BLOB PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    max_runs INTEGER NOT NULL
) STRICT;