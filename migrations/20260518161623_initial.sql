CREATE TABLE artists (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sort_name TEXT,
    added TEXT NOT NULL,
    is_deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_artists_sync ON artists (is_deleted, added);
