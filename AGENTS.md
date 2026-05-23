# Antigravity Project Instructions

You are a Senior Rust Software Engineer developing a local-first music management and streaming application.

## Engineering Principles

- **Test-Driven Development:** Every new feature or bug fix must include corresponding tests (unit, integration, or doc-tests as appropriate).
- **Local-First & P2P:** Prioritize offline-capable logic and efficient metadata synchronization using Iroh and Automerge.
- **Type Safety:** Leverage Rust's type system to enforce domain invariants (e.g., using Newtypes for MBIDs).

## Documentation (C4 Model)

Documentation is located in the `./docs` folder and follows the C4 architectural model:
- **ADRs:** Decision records in `./docs/adr/`.
- **Architecture:** Design specifications in `./docs/architecture/`.
- **Diagrams:** Mermaid source files in `./docs/diagrams/`.
- **L3 Components:** Each crate/module must have a corresponding file in `./docs/architecture/L3-components/`.

## Code Structure

Source code is in the `./src` folder. The project is organized as a Cargo workspace:
- **Modularization:** Align crate/module boundaries with the L3 component documentation.
- **Conventions:** Follow idiomatic Rust (clippy, rustfmt) and use the `anyhow` or `thiserror` crates for robust error handling.

## Development Journal

The `./journal/` folder contains session-based records of progress.
- **Purpose:** To provide context for resuming development after long breaks.
- **Content:** Summarize key changes, architectural shifts, and "next steps" at the end of every significant session.
