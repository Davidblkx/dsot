CREATE TABLE tracks (
    id BLOB PRIMARY KEY NOT NULL,
    release_id BLOB NOT NULL,
    recording_id BLOB NOT NULL,
    position INTEGER NOT NULL,
    disc_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_tracks_sync ON tracks (deleted, updated, created);

-- Index to enumerate tracks belonging to a release in disc/position order
CREATE INDEX idx_tracks_release ON tracks (release_id, disc_number, position);

-- Index to find where a recording appears across releases
CREATE INDEX idx_tracks_recording ON tracks (recording_id);

-- FTS5 virtual table for full-text search on tracks
CREATE VIRTUAL TABLE tracks_fts USING fts5(
    id UNINDEXED,
    title,
);

-- Triggers to keep tracks_fts in sync with tracks
CREATE TRIGGER tracks_after_insert AFTER INSERT ON tracks BEGIN
    INSERT INTO tracks_fts(id, title) VALUES (new.id, new.title);
END;

CREATE TRIGGER tracks_after_delete AFTER DELETE ON tracks BEGIN
    DELETE FROM tracks_fts WHERE id = old.id;
END;

CREATE TRIGGER tracks_after_update AFTER UPDATE ON tracks BEGIN
    DELETE FROM tracks_fts WHERE id = old.id;
    INSERT INTO tracks_fts(id, title) VALUES (new.id, new.title);
END;
