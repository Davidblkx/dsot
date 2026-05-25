CREATE TABLE releases (
    id BLOB PRIMARY KEY NOT NULL,
    release_group_id BLOB NOT NULL,
    title TEXT NOT NULL,
    barcode TEXT,
    release_date TEXT,
    format TEXT NOT NULL,
    label TEXT,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_releases_sync ON releases (deleted, updated, created);

-- Index to optimize "releases in this release group" lookups
CREATE INDEX idx_releases_release_group ON releases (release_group_id);

-- FTS5 virtual table for full-text search on releases.
-- Only `title` is indexed; `format` is enum-like and `barcode`/`label` are
-- better served by direct equality lookups than tokenized search.
CREATE VIRTUAL TABLE releases_fts USING fts5(
    id UNINDEXED,
    title,
);

-- Triggers to keep releases_fts in sync with releases
CREATE TRIGGER releases_after_insert AFTER INSERT ON releases BEGIN
    INSERT INTO releases_fts(id, title) VALUES (new.id, new.title);
END;

CREATE TRIGGER releases_after_delete AFTER DELETE ON releases BEGIN
    DELETE FROM releases_fts WHERE id = old.id;
END;

CREATE TRIGGER releases_after_update AFTER UPDATE ON releases BEGIN
    DELETE FROM releases_fts WHERE id = old.id;
    INSERT INTO releases_fts(id, title) VALUES (new.id, new.title);
END;
