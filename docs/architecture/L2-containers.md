# Container Architecture

This document describes the high-level container structure of the DSOT application. In our Rust codebase, these containers are represented by crates in a single Cargo workspace, dividing responsibilities between domain modeling, database persistence & sync journaling, proc-macro generation, and API integration.

## Container Diagram

```mermaid
graph TD
    subgraph Cargo Workspace [DSOT Crate Containers]
        DesktopApp[dsot_app]
        Lib[dsot_lib]
        Model[dsot_model]
        DbSync[dsot_db_sync]
        Derive[dsot_derive]
        MB[dsot_music_brainz]
        Config[dsot_config]
    end

    subgraph Storage [Local Storage Layer]
        SQLite[(SQLite DB + FTS5)]
        Redb[(redb Sync Journal)]
    end

    subgraph Remote [External API Services]
        MusicBrainzAPI[MusicBrainz API]
    end

    DesktopApp -->|Uses state & configurations| Lib
    Lib -->|Invokes CRUD & Search| DbSync
    Lib -->|Uses models| Model
    Lib -->|Loads configuration| Config
    
    DbSync -->|Serializes/Deserializes| Model
    Derive -.->|Generates SyncEntity & Repo logic for| Model
    
    DbSync -->|Executes SQLite queries & triggers FTS5| SQLite
    DbSync -->|Logs transactional SyncOperations| Redb
    
    DesktopApp -->|Fetches metadata| MB
    MB -->|JSON Web Requests| MusicBrainzAPI
```

---

## Workspace Containers

### 1. `dsot_model` (Domain & Entity Definition)
*   **Responsibility:** Defines the clean domain types (like `Artist`) and shared models.
*   **Technology:** Pure Rust, `serde`.
*   **State:** Stateless representation of data.

### 2. `dsot_db_sync` (Replication Journal & Repository)
*   **Responsibility:** Provides the storage engine, transaction management, full-text search interface, and sync journal.
*   **Technology:** `sqlx` (SQLite driver with FTS5 triggers), `redb` (embedded pure Rust Key-Value database), `blake3` (sync hashing).
*   **Key Interface:** `SyncEntityRepository` trait defining standard CRUD and FTS5 search queries.

### 3. `dsot_derive` (Procedural Macro Generator)
*   **Responsibility:** Reduces boilerplate by automatically generating repository structures, SQL CRUD bindings, and FTS5 search queries for any annotated struct.
*   **Technology:** `proc-macro2`, `syn`, `quote`.
*   **Key Macro:** `#[derive(SyncEntity)]` with custom attributes like `#[table(...)]`.

### 4. `dsot_music_brainz` (External Metadata Client)
*   **Responsibility:** Provides a type-safe API client to query the MusicBrainz database.
*   **Technology:** `reqwest`, `serde_json`, Lucene query-builder utilities.

### 5. `dsot_config` (Flexible Configuration Management)
*   **Responsibility:** Loads, parses, merges, and overrides multi-source, multi-layer application settings (defaults, files, custom paths, env).
*   **Technology:** `bakunin_config`, `dirs`, `thiserror`.

### 6. `dsot_lib` (Orchestration & State Management)
*   **Responsibility:** Integrates user management, logging, database connections, and application-wide configuration into a unified state container (`DsotState`).
*   **Technology:** `dsot_db_sync`, `dsot_config`, `dsot_model`, `sysdirs`, `fern`.

### 7. `dsot_app` (Multi-Platform Client Interface)
*   **Responsibility:** Renders the Dioxus-based user interface for managing libraries and syncing metadata. Runs natively on both desktop and mobile layouts.
*   **Technology:** `dioxus`, `dsot_lib`.
