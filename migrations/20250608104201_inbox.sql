-- Add migration script here
CREATE TABLE inbox (
    id BLOB PRIMARY KEY,
    title TEXT,
    artist TEXT,
    album TEXT,
    file TEXT,
    extra_info TEXT
);
