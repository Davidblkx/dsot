# Core Orchestration Component (`dsot_lib`)

The `dsot_lib` crate serves as the central orchestration, state management, and network engine for the DSOT application. It aggregates configuration parsing, multi-user workspace management, logger routing, database repository access, and peer-to-peer networking into a unified core container (`DsotCore`).

---

## Responsibility

- **Application Core Consolidation:** Enforces a clean domain interface by packaging configuration, repository interfaces, capabilities, user sessions, and network handlers into `DsotCore`.
- **Repository Interface:** Abstracts local and remote database access through the `DsotRepository` trait layer, allowing uniform queries whether the data source is a local SQLite/redb database or a remote peer.
- **P2P Networking (Iroh):** Initializes and manages an Iroh networking endpoint. Maintains the address book, device discovery, connection framing (`NetworkChannel`), and protocol transport (including DB synchronization) within the `network` module.
- **Global Logger Routing:** Coordinates multi-platform logger routing (sending logs to rolling files, standard error, or temporary system directories).
- **User Profile Management:** Manages individual user directories and credentials (`DsotUser`), maintaining absolute data partitioning (1 database per user).
- **Initialization Lifecycle:** Orchestrates the sequential startup chain from raw launch arguments to a fully hydrated `DsotCore` via `DsotCoreInitOptions`.
- **UI State Management:** Provides reactive state structures (`DsotState` containing `RemoteDevices` and `InboxState`) for binding into UI frameworks like Dioxus.

---

## Core Structures & Interfaces

### 1. `DsotCore`
The primary application context holding all services.

```rust
#[derive(Debug, Clone)]
pub struct DsotCore {
    /// Capabilities of the DSOT system.
    pub cap: Capability,
    /// Configuration for the DSOT system.
    pub config: Arc<DsotAppConfig>,
    /// Repository for the DSOT system.
    pub repo: DsotRepository,
    /// UI State for the DSOT system (devices, inbox).
    pub state: DsotState,
    /// Network for the DSOT system.
    pub net: DsotNetwork,
    /// Job manager for the DSOT system.
    pub jobs: JobManager,
    /// Current application user, can be empty
    pub user: DsotUser,
}
```

### 2. `DsotRepository`
Provides an abstraction over the underlying storage, routing requests to `LocalRepo`, `RemoteRepo`, or `NoopRepo` depending on capabilities and connections.

### 3. `DsotState`
A reactive state container designed for UI consumption, wrapping repository data such as `RemoteDevices` and `InboxState`.

### 4. `DsotCoreInitOptions`
A fluent builder configuration passed to the initialization chain.

```rust
pub struct DsotCoreInitOptions {
    pub debug: bool,
    pub config_file: Option<String>,
    pub cap: Capability,
}
```

---

## Startup Initialization Flow

When the client interface calls `DsotCoreInitOptions::initialize()`, the library coordinates the following startup steps:

1. **Early Logging Capture:** If debug is active, it initializes a trace logger immediately to capture startup warnings.
2. **Layered Config Retrieval:** Sources configurations using `dsot_config` and system capabilities.
3. **Final Logger Routing:** If not in debug mode, routes logger output to the paths/levels specified in the configuration.
4. **User Preparation:** Discovers and logs in the local user profile (`DsotUser`), targeting their isolated directory.
5. **Repository Binding:** Initializes `DsotRepository` against the user's data directory.
6. **State Hydration:** Initializes UI `DsotState` using the repository.
7. **Network Bootstrapping:** Starts the `DsotNetwork` Iroh node, mounting protocols (like database sync) and initiating discovery.
8. **Return Core:** Yields the fully hydrated `DsotCore`.
