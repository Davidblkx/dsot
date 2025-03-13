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
    name TEXT NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

CREATE INDEX artist_aliases_artist_id ON artist_aliases (artist_id);
