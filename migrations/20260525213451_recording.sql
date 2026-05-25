CREATE TABLE recordings (
    id BLOB PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    duration_ms INTEGER NOT NULL,
    isrc TEXT,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_recordings_sync ON recordings (deleted, updated, created);

-- FTS5 virtual table for full-text search on recordings.
-- Both `title` and `isrc` are indexed.
CREATE VIRTUAL TABLE recordings_fts USING fts5(
    id UNINDEXED,
    title,
    isrc,
);

-- Triggers to keep recordings_fts in sync with recordings
CREATE TRIGGER recordings_after_insert AFTER INSERT ON recordings BEGIN
    INSERT INTO recordings_fts(id, title, isrc) VALUES (new.id, new.title, new.isrc);
END;

CREATE TRIGGER recordings_after_delete AFTER DELETE ON recordings BEGIN
    DELETE FROM recordings_fts WHERE id = old.id;
END;

CREATE TRIGGER recordings_after_update AFTER UPDATE ON recordings BEGIN
    DELETE FROM recordings_fts WHERE id = old.id;
    INSERT INTO recordings_fts(id, title, isrc) VALUES (new.id, new.title, new.isrc);
END;
