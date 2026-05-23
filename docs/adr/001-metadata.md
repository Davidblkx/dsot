# Metadata (MusicBrainz + Deezer)

## Context & Problem Statement

We are developing a self-hosted, multi-device music manager and player written in Rust. The system must support metadata scraping, media binary storage, and cross-device synchronization. The application targets deployment profiles on resource-constrained infrastructure (e.g., home servers, NAS devices, or low-tier VPS) with a limited number of users. To ensure strict data privacy and security, each user possesses an isolated, independent database instance.

To achieve robust cross-device synchronization (e.g., syncing playlists, play counts, ratings, and file binaries between a home server and mobile clients), the system cannot rely on brittle metadata string matching (e.g., matching tracks by comparing `artist` or `album` strings). Naming conventions suffer from formatting drift, local tagging variations, and encoding differences across client devices. We need a deterministic metadata engine that can uniquely identify audio entities globally.

However, existing comprehensive open metadata engines (such as MusicBrainz) enforce strict upstream rate limits (1 request/second) that severely bottleneck the initial user onboarding experience when importing large, untagged libraries. Conversely, commercial streaming engines (such as Spotify) require users to register developer accounts to provision API keys, creating high initial setup friction.

We need to decide on the data sources and structural flow for metadata resolution that guarantees frictionless initial onboarding while establishing deterministic global IDs for cross-device sync.

---

## Decision Keys

We will implement a **Multi-Tiered Hybrid Metadata Pipeline** structured as an asynchronous fallback chain: **Local File Tags -> Deezer API [PLANNED] -> MusicBrainz API**.

1. **Frictionless Cosmetic Baseline [PLANNED]:** Use the **Deezer API** as the planned primary public network engine for instant track metadata and high-resolution artwork acquisition. It requires zero configuration, API keys, or user authentication.
2. **Global Synchronization Anchor:** Use **MusicBrainz IDs (MBIDs)** as the immutable, deterministic global keys to link tracks, artists, and albums across multiple user devices.
3. **Bridge Identifiers [PLANNED]:** Leverage industry-standard **ISRC (International Standard Recording Code)** and **UPC/Barcode** values returned by Deezer as exact query parameters to resolve MBIDs, bypassing ambiguous text-based searches.
4. **Decoupled Domain Engine:** Define the pipeline using an abstract, trait-driven interface layer in Rust to allow seamless support for optional providers (**Spotify** or **Discogs**) in future development phases.

---

## Technical Architecture & Lifecycle Workflow

The metadata pipeline splits data ingestion into two distinct priorities: **Immediate User Feedback** (UI/Cosmetic data) and **Lazy Consistency** (Sync Anchors).

```
                [Audio File Ingested into System]
                               │
                               ▼
            Step 1: Extract Local Tags (ID3/FLAC)
                               │
               ┌───────────────┴───────────────┐
               ▼                               ▼
     [Has MusicBrainz ID]            [No MusicBrainz ID]
               │                               │
               │                               ▼
               │                     Step 2: Query Deezer API [PLANNED]
               │                      - Fast, Unauthenticated
               │                      - Fetches Artwork & Metadata
               │                      - Extracts ISRC & Barcode
               │                               │
               ▼                               ▼
       [Anchor Verified]             Step 3: Queue MusicBrainz Task [PLANNED]
               │                      - Serialized Background Worker
               │                      - Queries MBID via ISRC/Barcode
               │                               │
               ▼                               ▼
       [Ready to Sync] ◄────────────── [Assign MBID to Track]
               │
               ▼
   [Persist to Per-User Isolated SQLite DB]

```

### 1. Ingestion Phase 1: Local Parsing

When a media binary is introduced, the system extracts embedded tags natively. If the file already possesses a valid `musicbrainz_id`, the track bypasses network resolution entirely and is flagged as immediate sync-ready.

### 2. Ingestion Phase 2: Frictionless Acceleration (Deezer API) [PLANNED]

If the file lacks global IDs, the system is planned to invoke the Deezer API. Because it does not require authentication tokens, it serves as an high-speed ingestion layer.

* This phase populates the user's isolated database with display essentials (clean titles, track ordering, and high-res cover art links) so the UI feels instantly responsive.
* Critically, it extracts the recording's unique **ISRC** and the release's **UPC/Barcode**.

### 3. Ingestion Phase 3: Resolution of the Sync Anchor (MusicBrainz API) [PLANNED]

Because the planned Deezer integration relies on closed proprietary IDs, its internal keys cannot be safely used as universal sync hooks across independent installations. The system redirects the fetched ISRC/Barcode metadata into a background processing queue.

* An asynchronous worker isolates upstream traffic to respect MusicBrainz’s mandatory 1 request per second rule.
* The worker uses the precise ISRC/Barcode values to retrieve the official **MBID** without performing ambiguous string parsing.
* Once the MBID is appended to the user's database record, the track is fully anchored and eligible for conflict-free multi-device synchronization.

---

## Consequences & Trade-offs

### Pros

* **Zero-Configuration Onboarding:** Users can deploy the system container or binary and experience rich metadata scraping immediately without navigating developer portals to generate credentials.
* **Deterministic Sync Foundation:** By referencing all synchronized track mutations (playlists, play counts, ratings) to a global `MBID`, client applications can seamlessly deduplicate and match tracks regardless of file-name or extension disparities.
* **Optimized Resource Footprint:** Separating fast media mapping (Deezer) from throttled metadata processing (MusicBrainz) prevents web-server thread starvation. The slow operations are offloaded into background tasks, conforming to Rust's async-concurrency safety principles.
* **Total Data Isolation:** The metadata pipeline feeds directly into independent database runtimes per user, ensuring zero leakage or inter-database entity pollution.

### Cons

* **Eventual Sync Availability:** For entirely untagged music libraries, files will display correct visual data instantly, but multi-device synchronization will compile lazily as the background background worker chunks through the 1 request/second limitation.
* **Upstream Interface Dependency:** Utilizing unauthenticated commercial schemas leaves a vulnerability if Deezer modifies their endpoints or locks down public metadata access. This risk is minimized by abstracting the ingestion pipelines behind decoupled interfaces, rendering future engine swaps simple to perform.
