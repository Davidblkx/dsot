# Selection of SQLite with FTS5 as Core Storage Engine for Local-First Music Manager

## Context and Problem Statement

We are building a local-first music management application designed to handle lightweight metadata indexing (Track, Artist, Album, Playlists) alongside large external binary audio assets (`.mp3`, `.flac`). The system must guarantee:

1. **Multi-device synchronization** over local networks, NAS, and cloud instances using a transactional operation-log replication journal (`redb`) compiling entity CRUD updates.
2. **Snappy read execution** for browsing, sorting, and filtering thousands of metadata rows locally.
3. **Full-Text Search (FTS)** capability (e.g., matching partial text like `"hotel calif"` across track parameters).
4. **Isolated data partitioning** on a strict **1-DB-per-user** boundary supporting up to 5 concurrent users per system installation.

We need to choose an embedded storage solution that balances systems optimization, low-latency search capabilities, compile-time type safety, and maximum multi-platform portability (Desktop, NAS, and future Mobile or WebAssembly/WASM targets).

---

## Decision Drivers

* **Cross-Platform Portability:** The engine must compile and execute safely across Linux daemons (NAS), Desktop operating systems (macOS/Windows via Tauri), and ideally work within mobile sandbox constraints (iOS/Android) or Web viewlayers (WASM) without triggering operating system memory violations.
* **Query Power & Full-Text Search Integration:** The system requires arbitrary structured queries (e.g., sorting tracking lists dynamically) coupled with performant, robust text search capabilities.
* **Crash Resilience & Transactional Safety:** Unexpected application shutdowns or power failure (especially on local client devices or cheap NAS systems) must not result in unrecoverable database file corruption.
* **Operational Ergonomics:** Avoid excessive architectural complexity, synchronization dual-write hazards, and binary bloat where possible.

---

## Considered Options

1. **SQLite (with FTS5 Virtual Extensions)** via `rusqlite` / `sqlx`
2. **The Pure Rust Modular Pair (Heed/LMDB + Tantivy)**
3. **SurrealDB (Embedded Rust Mode)**

---

## Decision Outcome

Chosen option: **Option 1: SQLite (with FTS5 Virtual Extensions)**.

SQLite strikes the definitive, most pragmatic balance between systems optimization and developer flexibility for a cross-platform deployment strategy. While pure Rust Key-Value pairs combined with standalone indexing libraries yield microsecond advantages on server hardware, they fail under the portability constraints of multi-platform client distribution.

---

## Pros and Cons of the Options

### Option 1: SQLite with FTS5

SQLite provides a single-file relational database engine coupled with an inverted index text module (`FTS5`).

* **Positive:** **Absolute Portability.** SQLite compiles natively to any target, including mobile architectures and WebAssembly via the Origin Private File System (`OPFS`). It has no runtime multi-threading assumptions that break in single-threaded WASM environments.
* **Positive:** **Atomicity over Dual-Writes.** By utilizing internal relational constraints, we can update indexed metadata columns, the virtual full-text search layout, and the local sync operation log inside a *single atomic database transaction*. This completely eliminates data-skew risks if the process crashes mid-write.
* **Positive:** **Industrial Crash Resilience.** SQLite's Write-Ahead Logging (`WAL`) mode offers decades of rigorous, real-world file system corruption protection during unexpected power termination events.
* **Negative:** Interfacing via C-bindings (`rusqlite`) requires a native C toolchain installed during compilation loops, slightly complicating pure-Rust cross-compilation configurations.

### Option 2: The Pure Rust Modular Pair (Heed/LMDB + Tantivy)

An explicit systems architecture mapping serialized sync byte arrays directly to Meilisearch's embedded Key-Value engine (`heed`) while manually piping text fields into Quickwit's search library (`tantivy`).

* **Positive:** Blistering speed. Zero SQL string compilation or abstraction overhead. Reads access the database values through bare pointers directly pointing to the operating system's page cache (`mmap`).
* **Negative:** **The Dual-Write Hazard.** Sled/Heed and Tantivy do not share a unified transactional log. A process crash midway through an update loop results in an unsynchronized data layer that requires building manual fallback reconciliation code.
* **Negative:** **Platform Limitations.** Memory-mapping (`mmap`) features face strict memory-footprint constraints on mobile systems (iOS), and `tantivy`'s deep dependency on native system threading model primitives limits its usage in web platform architectures (WASM).

### Option 3: SurrealDB (Embedded)

A multi-model database engine compiled directly into the Rust layer using an integrated SurrealQL parser.

* **Positive:** Exceptional developer ergonomics. Combines document nesting, relation links, and native full-text search syntax without mapping boilerplate DTO structures.
* **Negative:** Heavy binary overhead. The inclusion of embedded analytical modules and a JavaScript execution engine significantly bloats compile times and final application package footprints.
* **Negative:** Younger codebase profile. Its storage subsystem and index optimization paths lack the maturity of SQLite when running on lightweight or restricted client infrastructure.

---

## Implementation Strategy

To prevent vendor lock-in and keep our presentation components decoupled, our architecture enforces a strict separation between our core domain structures and the database representation.

### Data Layer Separation

The application frontend and database clients communicate solely using the pure domain structures (defined in `model`) and the generic repository interfaces (defined in `db_sync`). The SQLite database storage engine maps these structures internally into dedicated flat tables.

Whenever a user or synchronization trigger executes a write operation:
1. The domain structure is mapped to the corresponding SQL structure.
2. A `SyncOperation` (representing Create, Update, Delete, or Restore) is computed based on changes.
3. The SQL record and its corresponding serialized `SyncOperation` journal entry are committed inside a single, atomic SQLite transaction block, ensuring consistency.
4. SQLite triggers automatically propagate text changes to the FTS5 virtual tables, ensuring real-text index alignment.

This strategy guarantees a performant, zero-cost read pipeline for rendering responsive views while maintaining rock-solid synchronization histories beneath the hood.
