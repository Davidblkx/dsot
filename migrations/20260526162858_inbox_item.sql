CREATE TABLE inbox_items (
    id BLOB PRIMARY KEY NOT NULL,
    -- msgpack-encoded `InboxValue`. Opaque to SQL; decode application-side.
    value BLOB NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    -- Set when status = 'Resolved'. Caller infers the target table from the
    -- decoded InboxValue kind.
    resolved_id BLOB,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0
) STRICT;

-- Index to optimize syncing lookups and sorting
CREATE INDEX idx_inbox_items_sync ON inbox_items (deleted, updated, created);

-- Most common query is "show me my visible pending items".
-- Leads with `deleted` because the visible-list filter is always
-- WHERE deleted = 0 AND status = ?.
CREATE INDEX idx_inbox_items_status ON inbox_items (deleted, status);

-- FTS table exists only so the macro-generated search() query compiles.
-- The payload is opaque BLOB, so there's nothing meaningful to full-text
-- index. Triggers are intentionally omitted.
CREATE VIRTUAL TABLE inbox_items_fts USING fts5(
    id UNINDEXED,
    status
);
