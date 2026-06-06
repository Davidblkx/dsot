# Procedural Macro Component (`dsot_derive`)

The `dsot_derive` crate is a dedicated code generation library containing the `#[derive(SyncEntity)]` procedural macro. It eliminates repetitive SQL and serialization boilerplate for all domain entities.

## Macro Code Generation Responsibilities

When applied to a standard domain struct, the `#[derive(SyncEntity)]` macro automatically generates:

1.  **Relational SQL DTOs:**
    Generates a duplicate struct appended with the `Sql` suffix (e.g. `Artist` generates `ArtistSql`). This SQL representation contains database auditing columns (`created`, `updated`, `deleted` flags) required for operational synchronization.
2.  **Conversion Traits:**
    Implements standard `From` and `Into` traits to convert between the domain struct and its SQL representation seamlessly.
3.  **Sync Entity Primitives (`SyncEntity` trait):**
    Implements the `SyncEntity` trait, defining serialization methods (`to_bytes`, `from_bytes` using MessagePack) and calculating entity delta changes:
    *   `op_create()`: Wraps a serializable representation into a `SyncOperation::Create`.
    *   `op_update(prev)`: Inspects fields, compares them value-by-value with a previous state, and compiles a vector of `UpdateColumnOp` expressions wrapped inside a `SyncOperation::Update`.
4.  **Database Repository (`SyncEntityRepository` trait):**
    Implements the complete, asynchronous CRUD and search repository backend (appended with `SqlRepository`, e.g. `ArtistSqlRepository`) for `sqlx` connections.

---

## Macro-Generated Queries & Search Matching

The macro reads struct metadata attributes to dynamically construct SQL queries. For example, using the `#[table(artists)]` annotation:

### Generated CRUD Queries

*   **Insert:** Generates an `INSERT INTO artists ...` statement binding all fields.
*   **Get:** Generates `SELECT * FROM artists WHERE id = ? LIMIT 1`.
*   **List:** Generates pagination-ready query: `SELECT * FROM artists LIMIT ? OFFSET ?`.
*   **Update:** Dynamically builds assignment queries mapping incoming `UpdateColumnOp` updates into targeted `UPDATE artists SET ... WHERE id = ?` statements.
*   **Soft Delete:** Maps delete operations to a soft-delete write: `UPDATE artists SET deleted = 1, updated = ? WHERE id = ?`.
*   **Restore:** Restores soft-deleted entities: `UPDATE artists SET deleted = 0, updated = ? WHERE id = ?`.

### Generated FTS5 Search Query

The macro automatically implements the `search` method of the `SyncEntityRepository` trait by constructing a search query that joins the target entity table with its FTS5 virtual table using the binary UUID `id` column.

For `Artist` (table `artists`), the macro generates the following SQLite compilation query:

```sql
SELECT a.* 
FROM artists a 
JOIN artists_fts f ON a.id = f.id 
WHERE artists_fts MATCH ? AND a.deleted = 0 
ORDER BY f.rank;
```

This ensures that:
1.  **Snappy Search Performance:** Queries utilize the internal inverted search indices of the SQLite FTS5 virtual layout.
2.  **Safety & Filtering:** Soft-deleted records (`deleted = 1`) are excluded automatically.
3.  **Relevance Ranking:** Results are ordered by SQLite's BM25 search relevance rank (`f.rank`).
4.  **Type Safety:** The returned result set is parsed directly back into a vector of the strongly-typed domain model.
