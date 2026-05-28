CREATE TABLE track_files (
    id BLOB PRIMARY KEY NOT NULL,
    recording_id BLOB NOT NULL,
    file_hash BLOB NOT NULL UNIQUE,
    file_size INTEGER NOT NULL,
    format TEXT NOT NULL,
    uri TEXT NOT NULL,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_track_files_sync ON track_files (deleted, updated, created);

-- Index to find all local files for a given recording
CREATE INDEX idx_track_files_recording ON track_files (recording_id);

-- FTS table exists only so the macro-generated search() query compiles.
-- TrackFile has no natural text to full-text index; format/recording_id
-- lookups should use the regular WHERE clauses.
CREATE VIRTUAL TABLE track_files_fts USING fts5(
    id UNINDEXED,
    format
);
