-- Add migration script here
CREATE TABLE inbox (
    id BLOB PRIMARY KEY,
    user_id BLOB NOT NULL,
    title TEXT,
    artist TEXT,
    album TEXT,
    file TEXT,
    extra_info TEXT
);
