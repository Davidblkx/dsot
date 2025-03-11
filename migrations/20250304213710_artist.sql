-- Add migration script here
CREATE TABLE artists (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    sort_name TEXT,
    artist_type INTEGER NOT NULL
);

CREATE TABLE artist_aliases (
    id BLOB PRIMARY KEY,
    artist_id BLOB NOT NULL,
    name TEXT NOT NULL
);

CREATE INDEX artist_aliases_artist_id ON artist_aliases (artist_id);
