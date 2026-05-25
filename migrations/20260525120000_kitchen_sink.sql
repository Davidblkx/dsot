-- Schema for the KitchenSink test entity used by tests/sync_entity_contract.rs
-- in the dsot_db_sync crate. Exercises every column type the SyncEntity macro
-- supports (Text, nullable Text, Integer, Real, Blob, Json, Bool, Uuid).
-- No FTS triggers: search-via-FTS behavior is migration-specific and tested
-- against entities that own their own FTS triggers (e.g. artists).
CREATE TABLE kitchen_sinks (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    note TEXT,
    count INTEGER NOT NULL,
    ratio REAL NOT NULL,
    flag INTEGER NOT NULL,
    tags TEXT NOT NULL DEFAULT '[]',
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

CREATE INDEX idx_kitchen_sinks_sync ON kitchen_sinks (deleted, updated, created);

-- FTS table exists only so the macro-generated search() query compiles.
-- No triggers populate it; search() will return empty results.
CREATE VIRTUAL TABLE kitchen_sinks_fts USING fts5(
    id UNINDEXED,
    name
);
