# Sync & Database Component (`dsot_db_sync`)

The `dsot_db_sync` crate implements SQLite storage (`sqlx`) and a transactional operation-log synchronization engine backed by an embedded key/value store (`redb`). It provides:

- The runtime database handle (`DsotDatabase`) and its transactional view (`DsotDatabaseTransaction`).
- The contract every domain entity satisfies (`SyncEntity`, `SyncEntityRepository`) plus the diff/serialization machinery (`UpdateValue`, `EntityMessagePack`).
- The journal log and the primitives needed to compare and exchange journals between nodes.
- A `DatabaseManager` that opens an on-disk database folder, applies migrations, and manages backups.

The crate doesn't define any concrete entities — each entity lives in `dsot_model` and uses the `#[derive(SyncEntity)]` macro (in `dsot_derive`) to generate its `Sql` row struct + repository against this crate's contracts.

---

## Crate layout

```
src/modules/db_sync/src/
├── database/         # DsotDatabase + transactions + journal + per-entity ops
├── manager/          # On-disk folder lifecycle: open, migrate, backup/restore
├── model/            # SyncOperation, JournalEntry, UpdateValue, IntoUpdateValue
├── sync/             # Database synchronization protocol & message types (V1)
│   ├── mod.rs        # Re-exports V1 sync interfaces
│   └── v1/
│       ├── db_sync_node.rs    # DatabaseSyncNode wrapping database transactions & state
│       ├── error.rs           # SyncError enum for protocol errors
│       ├── handler.rs         # SyncNode trait + SyncNodeHandler execution loop
│       ├── mod.rs             # V1 module declarations & exports
│       └── model.rs           # DBSyncMessage models & binary serialization bindings
├── dser.rs           # EntityMessagePack — rmp-serde wrapper
├── entity.rs         # SyncEntity, IntoSyncEntity traits
├── error.rs          # DBSyncError — central error enum for the crate
├── registry.rs       # Distributed-slice registry of per-table apply functions
└── repo.rs           # SyncEntityRepository trait + ListQuery + RepositoryError (alias)
```

---

## Entity contract

### `SyncEntity` (`entity.rs`)

The macro implements this on every `*Sql` row struct. It produces the four mutation operations the journal carries:

```rust
pub trait SyncEntity {
    type Entity;

    fn get_id(&self) -> Uuid;
    fn op_create(&self) -> dser::Result<SyncOperation>;
    fn op_update(&self, prev: &Self::Entity) -> Option<SyncOperation>;
    fn op_delete(&self) -> SyncOperation;
    fn op_restore(&self) -> SyncOperation;
    fn from_bytes(data: &[u8]) -> dser::Result<Self::Entity>;
    fn to_bytes(&self) -> dser::Result<Vec<u8>>;
}
```

`op_update` returns `None` when nothing actually changed; otherwise it walks each non-id, non-metadata field and emits an `UpdateColumnOp` for each diff via `UpdateValue::get_if_diff` — plus a synthetic `updated = now()` column so the row's audit timestamp always advances.

`IntoSyncEntity` (also in `entity.rs`) converts the user-facing struct (e.g. `Artist`) into its persisted form (e.g. `ArtistSql`) — `Sql` rows carry the framework-managed `created` / `updated` / `deleted` columns that the user-facing struct doesn't.

### `SyncEntityRepository` (`repo.rs`)

```rust
pub trait SyncEntityRepository {
    type RepoEntity: SyncEntity<Entity = Self::RepoEntity>;

    fn get_table_name() -> &'static str;

    async fn insert(executor: &mut SqliteConnection, entity: &Self::RepoEntity) -> Result<()>;
    async fn get(executor: &mut SqliteConnection, id: Uuid) -> Result<Self::RepoEntity>;
    async fn try_get(executor: &mut SqliteConnection, id: Uuid) -> Result<Option<Self::RepoEntity>>;
    async fn list(executor: &mut SqliteConnection, q: ListQuery) -> Result<Vec<Self::RepoEntity>>;
    async fn update(executor: &mut SqliteConnection, id: Uuid, updates: Vec<UpdateColumnOp>) -> Result<()>;
    async fn delete(executor: &mut SqliteConnection, id: Uuid) -> Result<()>;
    async fn restore(executor: &mut SqliteConnection, id: Uuid) -> Result<()>;
    async fn search(executor: &mut SqliteConnection, query: String) -> Result<Vec<Self::RepoEntity>>;
    async fn exec_op(executor: &mut SqliteConnection, op: SyncOperation) -> Result<()>;
}
```

The macro implementation reads the `#[table(name)]` attribute on the source struct and emits compile-time-checked `sqlx::query!` / `sqlx::query_as!` calls — meaning the schema in `migrations/` must exist before the entity will compile.

`exec_op` dispatches a `SyncOperation` to the right concrete method (`Create` → `insert`, `Update` → `update`, `Delete` → `delete`, `Restore` → `restore`) and is the routine the journal replay uses.

`RepositoryError` is a type alias pointing to `DBSyncError`, which covers `EntityNotFound`, `DatabaseError` (sqlx), and `SerializationError` (msgpack).

---

## The journal

### `SyncOperation` (`model/mod.rs`)

```rust
pub enum SyncOperation {
    Create(Vec<u8>),                    // full row, msgpack-encoded
    Update(Uuid, Vec<UpdateColumnOp>),  // column-level diff
    Delete(Uuid),                       // soft-delete (sets deleted = 1)
    Restore(Uuid),                      // unsets the deleted flag
}

pub struct UpdateColumnOp {
    pub column: String,
    pub value: UpdateValue,
}
```

`Create` carries the entire encoded row; the id is embedded in the msgpack payload, not duplicated as an outer field.

### `UpdateValue` + `IntoUpdateValue` (`model/update_value.rs`)

`UpdateValue` is the SQLite-native value variant used by `Update` ops:

```rust
pub enum UpdateValue { Null, Integer(i64), Real(f64), Text(String), Blob(Vec<u8>) }
```

`IntoUpdateValue` is implemented for every Rust type that may appear on a `SyncEntity` field, plus an `Option<T>` blanket-impl mapping `None` to `Null`. Supported as of writing: `String`, `i64`, `u32`, `u64`, `f64`, `bool`, `chrono::DateTime<Utc>`, `chrono::NaiveDate`, `uuid::Uuid`, `Vec<u8>`, `sqlx::types::Json<Vec<String>>`. Any custom field type (e.g. the `ReleaseGroupType` enum in `dsot_model`) must provide its own `IntoUpdateValue` impl alongside its `sqlx::Encode<Sqlite>` / `sqlx::Decode<Sqlite>` impls.

### `EntityMessagePack` (`dser.rs`)

Thin wrapper over `rmp-serde` used for *all* on-wire and on-disk serialization in this crate (the entity row inside `Create`, the full `JournalEntry`, opaque payloads in any entity that carries `Vec<u8>`). Centralizing the format here means the choice of msgpack is changeable in one place.

### `JournalEntry` (`model/mod.rs`)

```rust
pub struct JournalEntry {
    pub id: Uuid,       // primary key in the redb journal table
    pub table: String,  // routes the op to a RepositoryRegistry entry
    pub op: SyncOperation,
}
```

### `redb` journal storage (`database/journal.rs`)

A single redb table, `JOURNAL_TABLE`, keyed by the journal entry's `Uuid` (`[u8; 16]`) and storing the msgpack-encoded `JournalEntry`. Every mutation that goes through `DsotDatabase` or `DsotDatabaseTransaction` writes a journal entry inside the same transaction as the SQL change, so the journal and SQL never drift.

---

## Synchronization

DSOT is local-first: each device owns a full SQLite database and a full journal. Devices reconcile by exchanging *journal entries*, not row snapshots, so a per-column edit on one device round-trips as a per-column edit on another.

### Protocol Engine (`sync/v1/`)

To establish a strict separation of concerns between database state reconciliation and network communication, `dsot_db_sync` contains zero networking dependencies or Iroh transport bindings. Instead, it exposes the core `SyncNode` trait and a unified protocol message enum (`DBSyncMessage`).

#### Core Message Types (`sync/v1/model.rs`)

All protocol interactions are modeled by the single enum **`DBSyncMessage`**:
*   `Hello(String)`: Always sent by the initiating node to advertise its database identifier (`db_id`).
*   `Validate(SyncHash)`: Sent to verify whether both nodes share the identical state hash (a `BLAKE3` digest over sorted journal keys).
*   `BeginExchange(Vec<SyncKey>)`: Initiates key exchange by sending all local journal keys (`[u8; 16]`).
*   `Exchange { keys, request, entries }`: The bidirectional reconciliation payload. Carries the sender's current journal keys (`keys`), keys requested from the remote node (`request`), and serialized journal entries (`entries`) requested in the previous turn.
*   `Completed`: Sent when both nodes confirm identical state hashes or complete their exchange.
*   `Error(SyncError)`: Sent when a node encounters an error (serialization failure, database ID mismatch, etc.).

#### Decoupled SyncNode Architecture

The sync subsystem decouples network transport from database state transitions using the `SyncNode` trait:

```mermaid
classDiagram
    class SyncNode {
        <<interface>>
        +get_db_id() Option~String~
        +handle(message) DBSyncMessage
    }
    class DatabaseSyncNode {
        +db DsotDatabase
        +trx DsotDatabaseTransaction
        +create(db) DatabaseSyncNode
        +close() bool
    }
    class NetworkDBSyncNode {
        +channel NetworkChannel
        +db_id Option~String~
        +start_sync(conn, db_id) NetworkDBSyncNode
    }
    SyncNode <|.. DatabaseSyncNode : in dsot_db_sync
    SyncNode <|.. NetworkDBSyncNode : in dsot_network
```

*   **`SyncNode`**: The async interface defining database ID retrieval and message processing.
*   **`DatabaseSyncNode`**: Resides in `dsot_db_sync`. Wraps a local `DsotDatabase` and an active `DsotDatabaseTransaction`. Implements `SyncNode` to evaluate differences against remote keys, apply incoming journal entries in a transaction, and retrieve requested journal entries from SQLite/redb storage.
*   **`SyncNodeHandler`**: Coordinates protocol execution between any two implementations of `SyncNode` via the loop function:
    ```rust
    pub async fn sync<NodeA: SyncNode, NodeB: SyncNode>(a: &mut NodeA, b: &mut NodeB) -> Result<()>
    ```
*   **`NetworkDBSyncNode` & `DBSyncProtocol`**: Reside in `dsot_network`. They implement `SyncNode` over an Iroh network connection (`NetworkChannel`) and register as an Iroh `ProtocolHandler` under ALPN `b"/dsot/db_sync/1"`. When a connection is accepted or initiated, `dsot_network` instantiates both `DatabaseSyncNode` (local) and `NetworkDBSyncNode` (remote) and runs `SyncNodeHandler::sync(&mut local, &mut remote)`.

#### Reconciliation Protocol Flow

When two nodes reconcile via `SyncNodeHandler::sync`, the following loop executes:

1.  **Session Handshake:** The initiating node sends `DBSyncMessage::Hello(db_id)`. The receiving node validates the ID against its local database. If IDs match, it computes its local state hash and replies with `DBSyncMessage::Validate(local_hash)`.
2.  **Hash Verification:** If the received hash matches the initiator's local hash, both databases are in sync and the initiator replies with `DBSyncMessage::Completed`.
3.  **Key Exchange Start:** If hashes mismatch, the initiator sends `DBSyncMessage::BeginExchange(local_keys)`.
4.  **Symmetric Merging & Trading:** Upon receiving `BeginExchange` or `Exchange`, the receiving node:
    *   Applies any incoming `entries` inside its transaction via `RepositoryRegistry::instance().apply()`.
    *   Computes the keys it is missing from `keys` (`get_keys_not_in_journal`).
    *   Fetches the journal entries requested by the remote node (`get_journal_entries_in_array`).
    *   If no entries need to be sent and no keys are missing, it sends `DBSyncMessage::Validate(hash)` to re-check synchronization. Otherwise, it sends `DBSyncMessage::Exchange { keys, request, entries }`.
5.  **Completion:** Once validation confirms identical state hashes, nodes exchange `DBSyncMessage::Completed` and the transaction is committed via `.close()`.

### Status

The database synchronization logic, state comparison, key exchange, payload exchange, and transaction replay primitives are fully implemented and verified in `dsot_db_sync`.

The external **network communication layer** — including Iroh connection management, framing (`NetworkChannel`), and protocol handling (`DBSyncProtocol`) — is cleanly decoupled and implemented in `dsot_network`.

### `RepositoryRegistry` (`registry.rs`)

Journal replay needs to find the right `SyncEntityRepository` impl for an arbitrary table name read out of a `JournalEntry`. The macro-generated code emits, per entity, a `static` entry into a `linkme::distributed_slice` named `APPLY_SQL_OPERATION_REF`:

```rust
pub struct ApplySqlOperationRef {
    pub table: &'static str,
    pub apply: ApplySqlOperation,  // fn(&mut Trx, SyncOperation) -> BoxFuture<Result<()>>
}

#[linkme::distributed_slice]
pub static APPLY_SQL_OPERATION_REF: [ApplySqlOperationRef];
```

`RepositoryRegistry::instance()` lazily builds a `HashMap<table_name, apply_fn>` from this slice at first use. **Consequence:** entities not linked into a given binary won't appear in the registry — handy for test binaries that intentionally include only a subset, but a footgun if you forget to depend on `dsot_model` from a binary that must replay all journals. `apply_journals_bytes` errors out with `RepositoryNotFound(table)` if it ever encounters a table no one registered for.

### Apply semantics

`safe_apply_op` (`database/entity_ops.rs`) is the routine every journal-driven write goes through. Its one nontrivial behavior: for a `Create`, it first checks whether the entity id already exists and **skips** the insert if so, instead of erroring on a duplicate-key violation. This is how the system stays idempotent under journal replay — applying the same journal twice is a no-op.

---

## Full-text search (FTS5)

For each searchable relational table, a parallel FTS5 virtual table holds the row's searchable columns. The FTS table is **populated by SQL triggers**, not by application code — so an `INSERT` / `UPDATE` / `DELETE` against the base table is automatically reflected in FTS within the same SQL statement.

Example (`migrations/20260518161623_artists.sql`):

```sql
CREATE VIRTUAL TABLE artists_fts USING fts5(
    id UNINDEXED,
    name,
    sort_name,
    aliases
);

CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
    INSERT INTO artists_fts(id, name, sort_name, aliases)
    VALUES (
        new.id, new.name, new.sort_name,
        (SELECT group_concat(value, ' ') FROM json_each(new.aliases))
    );
END;

CREATE TRIGGER artists_after_delete AFTER DELETE ON artists BEGIN
    DELETE FROM artists_fts WHERE id = old.id;
END;

CREATE TRIGGER artists_after_update AFTER UPDATE ON artists BEGIN
    DELETE FROM artists_fts WHERE id = old.id;
    INSERT INTO artists_fts(id, name, sort_name, aliases)
    VALUES (new.id, new.name, new.sort_name,
            (SELECT group_concat(value, ' ') FROM json_each(new.aliases)));
END;
```

Note the **DELETE + INSERT** pattern in the UPDATE trigger — the FTS5-recommended approach when an update touches indexed columns (cheaper and more robust than per-column updates on the virtual table). The `group_concat` over `json_each` flattens a `Json<Vec<String>>` column into space-separated tokens before indexing.

The macro-generated `search()` query joins the base table against `<table>_fts` on `id`, filters `MATCH ?` plus `deleted = 0`, and orders by `f.rank`. Soft-deleted rows stay in the FTS index (the after-delete trigger only fires on actual SQL `DELETE`, not on the `deleted = 1` flag flip) but the `WHERE` clause hides them from results.

Entities with no natural text to index still need an FTS table to exist for the generated `search()` query to compile — see `migrations/20260526143855_track_file.sql` for the minimal bare-FTS-table-without-triggers pattern.

---

## Runtime entry points

### `DsotDatabase` (`database/mod.rs`)

The runtime handle. Wraps a `redb::Database` (journal) and a `sqlx::SqlitePool` (SQL).
*   **Identity:** Carries an `id: String` representing the database identity. Can be customized on creation using `.with_id(id)` and retrieved using `.get_id()`.
*   **Convenience APIs:** Exposes per-entity methods that begin a transaction, apply + journal one operation, and commit atomically: `insert`, `update`, `upsert`, `delete`, `restore`, and `apply_journal`.
*   **Read-Only APIs:** Exposes connection pool accessors for read actions: `get`, `try_get`, `list`, and FTS5 `search`.

### `DsotDatabaseTransaction` (`database/transaction.rs`)

Returned from `DsotDatabase::begin_transaction()` when multiple operations must commit or roll back atomically. Holds a `redb::WriteTransaction` and a `sqlx::Transaction<Sqlite>` in lockstep. It provides the same per-entity mutation methods as `DsotDatabase` (minus the auto-committing behavior). The transaction must be finalized by calling `commit().await` or `rollback().await`.

### `DatabaseManager` (`manager/`)

A lightweight entry point that owns and manages a physical database directory on disk:

*   `open_folder(path)`: Ensures the directory exists and validates that it is a directory.
*   `open_database()`: Connects to `<dir>/library.sqlite`, runs the embedded `sqlx` migrations (`sqlx::migrate!("../../../migrations")`), opens the transactional key/value journal `<dir>/library.journal` using `redb`, and returns an initialized `DsotDatabase`.
*   `create_backup()`: Safely copies the live `.sqlite` and `.journal` files into `<dir>/backups/` with a `<name>__<uuid>` (UUID v7) suffix.
*   `get_backups()`: Scans the backups directory, verifying both `.sqlite` and `.journal` backup files exist (`is_valid()`), and returns a list of available `DatabaseBackup` items.
*   `DatabaseBackup::restore()`: Overwrites the active active database and journal files with the backup files.

Used by application code as the single "open the database" call site.

## Errors

- `DBSyncError` (`src/modules/db_sync/src/error.rs`) — the single, consolidated error enum for the crate. It represents the union of all errors that can occur within database synchronization and management, including:
  - `redb` storage, transaction, commit, table, and database errors.
  - `sqlx` / SQLite errors.
  - Serialization / Deserialization (`rmp_serde`) errors.
  - Standard IO errors.
  - SQLx migration errors.
  - Entity-specific errors (e.g. `EntityNotFound`, `TableMissmatchError`, `RepositoryNotFound`).
  - Sync-specific errors (`SyncError`, `PathIsNotAFolder`).
