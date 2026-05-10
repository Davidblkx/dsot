# DSOT Architecture Overview

DSOT (Digital Sound Organizer Tool) is a music management system designed with synchronization and multi-instance support in mind.

## Project Structure

The project is organized as a Rust workspace with several crates:

- **`core`**: The heart of the application.
  - **Models**: Defines domain entities (Artist, Album, Track, etc.) and the Journaling system.
  - **Storage**: Abstractions for data persistence.
    - `sql`: Main relational data using SQLite (via `sqlx`).
    - `kv`: Journaling system using `redb` and `bincode`.
- **`runtime`**: Manages the application lifecycle and shared state.
  - Initialized with a `Config`.
  - Holds the `DatabaseHandler` and logging handles.
  - Passed as a dependency to CLI commands and the server.
- **`music_brainz`**: A dedicated crate for interacting with the MusicBrainz API for metadata retrieval.
- **`server`**: An `axum`-based web server that exposes the system's functionality via an API.
- **`cli`**: A command-line interface for managing the system, including starting the server.
- **`client`**: A modern frontend built with Vue 3 and TypeScript using JSX.
  - Supports both Web and Desktop (via Tauri).
  - Uses Pinia for state management and Ark UI for components.
  - Uses a platform abstraction layer to handle environment-specific logic.

## Data Persistence & Synchronization

DSOT uses a dual-storage approach to enable robust synchronization between different instances:

1.  **SQLite (`library.db`)**: Stores the current state of all entities. This is the primary database for querying and UI display.
2.  **redb (`library.jrn`)**: A Key-Value store used for the **Journaling System**.

### Journaling System

Every change (Create, Update, Delete) to an entity is recorded as a **Journal Entry** in the `.jrn` file.
- **Entry ID**: Uses `uuid7` to ensure entries are globally unique and naturally ordered by time.
- **Action**: The type of change performed.
- **Entity & ID**: Identifies what was changed.
- **Body**: The serialized state of the entity at the time of the change (JSON or Bincode).

### Sync Mechanism

Synchronization works by replaying journal entries between instances.
- Each instance has a unique ID.
- Instances track the last applied journal entry from other instances to avoid duplicates and maintain consistency.
- The system supports "clean up" of old journal entries once they have been successfully propagated to all known instances.

## Key Technologies

- **Language**: Rust (Backend), TypeScript/Vue 3 (Frontend).
- **Web Framework**: Axum (Rust), Vite (Frontend).
- **Desktop Wrapper**: Tauri.
- **Databases**: SQLite (Relational), redb (KV/Journal).
- **API Integration**: MusicBrainz.
- **Serialization**: Serde, Bincode.
