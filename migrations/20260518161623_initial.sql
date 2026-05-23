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

-- FTS5 virtual table for full-text search on artists
CREATE VIRTUAL TABLE artists_fts USING fts5(
    id UNINDEXED,
    name,
    sort_name
);

-- Triggers to keep artists_fts in sync with artists
CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
    INSERT INTO artists_fts(id, name, sort_name) VALUES (new.id, new.name, new.sort_name);
END;

CREATE TRIGGER artists_after_delete AFTER DELETE ON artists BEGIN
    DELETE FROM artists_fts WHERE id = old.id;
END;

CREATE TRIGGER artists_after_update AFTER UPDATE ON artists BEGIN
    UPDATE artists_fts SET name = new.name, sort_name = new.sort_name WHERE id = old.id;
END;
