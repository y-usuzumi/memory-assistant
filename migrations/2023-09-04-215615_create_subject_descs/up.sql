-- Your SQL goes here
CREATE TABLE subject_descs (
    id BLOB PRIMARY KEY,
    title TEXT,
    description TEXT,
    max_runs INTEGER
) STRICT;