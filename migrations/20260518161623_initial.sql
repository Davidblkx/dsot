CREATE TABLE artists (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sort_name TEXT,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_artists_sync ON artists (deleted, updated, created);
