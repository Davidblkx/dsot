-- Generated migration for FTS5 table inbox_fts
CREATE VIRTUAL TABLE inbox_fts USING fts5 (
    id UNINDEXED,
    "title",
    "artist",
    "album",
    "file",
    "extra_info",
);

-- Triggers to keep the FTS5 table in sync
CREATE TRIGGER inbox_ai AFTER INSERT ON inbox BEGIN
    INSERT INTO inbox_fts (id, "title", "artist", "album", "file", "extra_info")
    VALUES (new.id, new."title", new."artist", new."album", new."file", new."extra_info");
END;

CREATE TRIGGER inbox_ad AFTER DELETE ON inbox BEGIN
    INSERT INTO inbox_fts (inbox_fts, id, "title", "artist", "album", "file", "extra_info")
    VALUES ('delete', old.id, old."title", old."artist", old."album", old."file", old."extra_info");
END;

CREATE TRIGGER inbox_au AFTER UPDATE ON inbox BEGIN
    INSERT INTO inbox_fts (inbox_fts, id, "title", "artist", "album", "file", "extra_info")
    VALUES ('delete', old.id, old."title", old."artist", old."album", old."file", old."extra_info");
    INSERT INTO inbox_fts (id, "title", "artist", "album", "file", "extra_info")
    VALUES (new.id, new."title", new."artist", new."album", new."file", new."extra_info");
END;

-- Initial population of the FTS5 table
INSERT INTO inbox_fts (id, "title", "artist", "album", "file", "extra_info")
SELECT id, "title", "artist", "album", "file", "extra_info" FROM inbox;
