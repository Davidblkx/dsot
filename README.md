# DSOT - Music Manager

DSOT is a local-first music management and streaming application designed to allow multiple devices to share metadata and music files.

## Philosophy

*   **Local-First:** All library data is stored and queried locally. The network is an enhancement for synchronization, not a requirement for playback or organization.
*   **Decentralized:** There is no central authority for user data. Data synchronization is planned to happen Peer-to-Peer (P2P) via **Iroh**.
*   **Operation Journaling:** Sync state is managed using a transactional operation-log journal (`redb`) that records database mutations dynamically, allowing clean replay and deduplication across devices.
*   **Search-Centric:** Instant results using integrated SQLite FTS5 (Full Text Search) virtual tables.

---

## Codebase Architecture

The project is structured as a Cargo workspace with four main crates:

1.  **`dsot_model` ([src/model](file:///projects/refactor/src/model))**: Defines the core domain entities (such as `Artist`) and shared models.
2.  **`dsot_db_sync` ([src/db_sync](file:///projects/refactor/src/db_sync))**: Implements database persistence using SQLite (`sqlx`) and transactional operation journaling using `redb`.
3.  **`dsot_derive` ([src/derive](file:///projects/refactor/src/derive))**: A procedural macro library generating serialization mapping, synchronization behaviors, and repository implementations (`#[derive(SyncEntity)]`) for domain entities.
4.  **`dsot_music_brainz` ([src/music_brainz](file:///projects/refactor/src/music_brainz))**: A type-safe client library to query and lookup entities via the MusicBrainz API.

---

## Development

### Prerequisites

*   Rust toolchain (2024 edition)
*   SQLite library installed on the host system (required by `sqlx`)

### Building and Testing

To compile the entire workspace:
```bash
cargo build
```

To run all unit and integration tests:
```bash
cargo test
```
