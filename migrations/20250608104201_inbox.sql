CREATE TABLE inbox (
    id BLOB PRIMARY KEY,
    title TEXT,
    artist TEXT,
    album TEXT,
    file TEXT,
    extra_info TEXT
);

CREATE TABLE users (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE INDEX users_names ON users (name);
