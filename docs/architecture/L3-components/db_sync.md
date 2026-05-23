# Sync & Database Component (`dsot_db_sync`)

The `dsot_db_sync` crate implements SQLite database storage (`sqlx`) and a transactional operation-log synchronization engine using an embedded Key-Value store (`redb`).

## Repository Architecture

Rather than a separate implementation per entity, `dsot_db_sync` establishes a generic database interaction layer driven by the `SyncEntityRepository` trait. Implementations are dynamically generated at compile-time by procedural macros.

### The `SyncEntityRepository` Trait

```rust
pub trait SyncEntityRepository {
    type RepoEntity: SyncEntity<Entity = Self::RepoEntity>;

    fn get_table_name() -> &'static str;

    async fn insert(executor: &mut SqliteConnection, entity: &Self::RepoEntity) -> Result<()>;
    async fn get(executor: &mut SqliteConnection, id: Uuid) -> Result<Self::RepoEntity>;
    async fn try_get(executor: &mut SqliteConnection, id: Uuid) -> Result<Option<Self::RepoEntity>>;
    async fn list(executor: &mut SqliteConnection, query: ListQuery) -> Result<Vec<Self::RepoEntity>>;
    async fn update(executor: &mut SqliteConnection, id: Uuid, updates: Vec<UpdateColumnOp>) -> Result<()>;
    async fn delete(executor: &mut SqliteConnection, id: Uuid) -> Result<()>;
    async fn restore(executor: &mut SqliteConnection, id: Uuid) -> Result<()>;
    async fn search(executor: &mut SqliteConnection, query: String) -> Result<Vec<Self::RepoEntity>>;
    async fn exec_op(executor: &mut SqliteConnection, op: SyncOperation) -> Result<()>;
}
```

---

## Synchronization Architecture

Replication between client devices and isolated nodes is achieved through an operation-log journaling engine. 

### 1. The `redb` Journal Log

A dedicated Key-Value database file using `redb` acts as the persistent, local-first transaction log of all entity mutations.
*   **Table Definition:** `JOURNAL_TABLE` maps unique UUIDs (`[u8; 16]`) of operation payloads to serialized MessagePack byte arrays representing the corresponding `SyncOperation`.
*   **Operations (`SyncOperation`):**
    *   `Create(id, MsgPackBytes)`: Inserts a new record.
    *   `Update(id, Vec<UpdateColumnOp>)`: Updates specific columns on an existing record.
    *   `Delete(id)`: Soft-deletes a record.
    *   `Restore(id)`: Restores a soft-deleted record.

### 2. Synchronization Handshake & BLAKE3 Hashes

When two DSOT nodes handshake to synchronize, they avoid transmitting entire database payloads by comparing a cryptographic summary hash of their journals:
1.  **State Hash:** A `BLAKE3` hasher iterates in order over all keys stored in the local `redb` journal and computes a single `[u8; 32]` finalized hash.
2.  **Comparison:** If the state hashes match, the nodes are fully in sync and terminate immediately.
3.  **Delta Resolution:** If the hashes differ, nodes exchange their log keys, determine missing operations via `get_keys_not_in_journal`, and exchange only the missing `SyncOperation` payloads, applying them locally via `exec_op` inside an exclusive SQL transaction.

---

## Full-Text Search (FTS5) Engine

DSOT implements snappy, responsive search directly in SQLite using virtual tables and automated synchronization triggers.

### FTS5 Virtual Table Mapping

For each searchable relational table, a parallel virtual table is created using the `fts5` extension. For example, for the `artists` table:
*   **Relational Columns:** `id` (BLOB primary key), `name` (TEXT), `sort_name` (TEXT), `created` (TEXT), `updated` (TEXT), `deleted` (INTEGER).
*   **FTS5 Virtual Columns:** `id` marked as `UNINDEXED` (stores the raw UUID), along with text search fields `name` and `sort_name`.

### Automatic SQL Triggers

To prevent synchronization dual-write bugs, the relational SQLite database maintains automatic SQL triggers. Any change to the main relational table is instantly and atomically propagated to the FTS5 virtual table:

```sql
-- Propagation on INSERT
CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
    INSERT INTO artists_fts(id, name, sort_name) VALUES (new.id, new.name, new.sort_name);
END;

-- Propagation on DELETE
CREATE TRIGGER artists_after_delete AFTER DELETE ON artists BEGIN
    DELETE FROM artists_fts WHERE id = old.id;
END;

-- Propagation on UPDATE
CREATE TRIGGER artists_after_update AFTER UPDATE ON artists BEGIN
    UPDATE artists_fts SET name = new.name, sort_name = new.sort_name WHERE id = old.id;
END;
```

When performing a search, the generated repository joins the relational table with the FTS5 table via `id` to return full, structured entity DTOs sorted by matching relevance (`rank`).
