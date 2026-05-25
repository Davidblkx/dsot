CREATE TABLE release_groups (
    id BLOB PRIMARY KEY NOT NULL,
    artist_id BLOB NOT NULL,
    title TEXT NOT NULL,
    primary_type TEXT NOT NULL,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_release_groups_sync ON release_groups (deleted, updated, created);

-- Index to optimize "release groups by artist" lookups
CREATE INDEX idx_release_groups_artist ON release_groups (artist_id);

-- FTS5 virtual table for full-text search on release groups.
-- Only `title` is indexed; `primary_type` is enum-like and better used as a
-- regular WHERE filter than as an FTS column.
CREATE VIRTUAL TABLE release_groups_fts USING fts5(
    id UNINDEXED,
    title,
);

-- Triggers to keep release_groups_fts in sync with release_groups
CREATE TRIGGER release_groups_after_insert AFTER INSERT ON release_groups BEGIN
    INSERT INTO release_groups_fts(id, title) VALUES (new.id, new.title);
END;

CREATE TRIGGER release_groups_after_delete AFTER DELETE ON release_groups BEGIN
    DELETE FROM release_groups_fts WHERE id = old.id;
END;

CREATE TRIGGER release_groups_after_update AFTER UPDATE ON release_groups BEGIN
    DELETE FROM release_groups_fts WHERE id = old.id;
    INSERT INTO release_groups_fts(id, title) VALUES (new.id, new.title);
END;
