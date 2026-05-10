# DSOT - Music Manager

DSOT is local first music management and streaming. It allow multiple devices to share metadata and music files between them.

## Philosophy

- Local-First: Data is stored and queried locally in SQLite. The network is an enhancement, not a requirement.
- Decentralized: No central authority for user data. Synchronization happens P2P via Iroh.
- Conflict-Free: State is managed via Automerge (CRDTs) to handle concurrent edits across devices.
- Search-Centric: Instant results using SQLite FTS5.
