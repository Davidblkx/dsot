-- Add migration script here
CREATE TABLE artists (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    sort_name TEXT,
);
