CREATE TABLE track_files (
    id BLOB PRIMARY KEY NOT NULL,
    recording_id BLOB NOT NULL,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_track_files_sync ON track_files (deleted, updated, created);

-- Index to find all local files for a given recording
CREATE INDEX idx_track_files_recording ON track_files (recording_id);

-- FTS table exists only so the macro-generated search() query compiles.
-- TrackFile has no natural text to full-text index; recording_id lookup
-- should use the regular WHERE clauses.
CREATE VIRTUAL TABLE track_files_fts USING fts5(
    id UNINDEXED,
    recording_id
);
