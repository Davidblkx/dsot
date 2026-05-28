# Domain Model Component (`dsot_model`)

The `dsot_model` crate defines the pure, stateless domain entities and shared data models for the DSOT application. It acts as the shared vocabulary across the entire system.

## Key Design Patterns

### ID Generation & Matching Strategy

To ensure seamless synchronization across different user devices without relying on a central database, DSOT uses a dual-identity strategy:

1.  **Matched Media (Global Sync Anchor):**
    When metadata is queried and matched from public databases (like MusicBrainz), the entity's primary key `id` stores the official MusicBrainz UUID (MBID). Because these IDs are globally unique and deterministic, when multiple devices independently look up and add the same artist or album, their local databases will naturally merge the metadata during sync because the IDs match perfectly.
2.  **Unmatched/Local Media:**
    If a media file is indexed from local tags and cannot be verified via MusicBrainz, the system assigns a generated UUID (e.g. `Uuid::now_v7()`). These records represent local unmatched states. If the user later performs a match lookup, the unmatched local entity is merged into a newly retrieved MBID-anchored entity.
3.  **TrackFile Binaries:**
    Personal audio binaries dedupe on their **SHA-256 content hash**. Each `TrackFile` still has a regular `Uuid` primary key, but the schema enforces `UNIQUE(file_hash)` so two devices that independently index the same physical file converge to a single row at sync time.

---

## Active Entities

### Artist
The `Artist` entity represents a musician, group, or orchestra.

```rust
#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(artists)]
pub struct Artist {
    /// MusicBrainz MBID if matched; otherwise a generated Uuid.
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub aliases: Json<Vec<String>>,
}
```

---

### ReleaseGroup (The Abstract Album)
Represents the logical grouping of releases (e.g., "The Dark Side of the Moon" as a concept).

```rust
#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(release_groups)]
pub struct ReleaseGroup {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub primary_type: ReleaseGroupType,
}
```

`primary_type` is stored as TEXT. The `Custom(String)` variant is a catch-all: any DB value that doesn't match a known variant decodes into `Custom(...)`, so the application never fails to load a row with an unexpected value (legacy data, future variants this build doesn't know yet, etc.).

```rust
pub enum ReleaseGroupType {
    Album,
    Single,
    EP,
    Broadcast,
    Live,
    Other,
    Unknown,         // default; "no classification yet"
    Custom(String),  // fallback for unrecognized strings
}
```

### Release (The Specific Pressing)
Represents a specific physical or digital pressing of an album (e.g., the 1993 Remaster or a UK Vinyl release).

```rust
#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(releases)]
pub struct Release {
    pub id: Uuid,
    pub release_group_id: Uuid,
    pub title: String,
    pub barcode: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub format: String,
    pub label: Option<String>,
}
```

### Recording (The Audio Mix)
Represents a unique audio mix or master track.

```rust
#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(recordings)]
pub struct Recording {
    pub id: Uuid,
    pub title: String,
    pub duration_ms: u32,
    pub isrc: Option<String>,
}
```

### Track (The Album Position Link)
Links a specific `Recording` to a position on a `Release`. The MusicBrainz Track ID is distinct from the Recording ID — the same recording can appear on multiple releases with different track IDs.

```rust
#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(tracks)]
pub struct Track {
    pub id: Uuid,
    pub release_id: Uuid,
    pub recording_id: Uuid,
    pub position: u32,    // track number within the disc
    pub disc_number: u32, // 1-based; multi-disc releases use 2, 3, ...
    pub title: String,
}
```

### TrackFile (Local Binary File)
Links a local physical audio file on the user's filesystem to a domain `Recording`. Dedup is enforced by a `UNIQUE(file_hash)` constraint at the schema level rather than by using the hash as a primary key — see the [ID Generation](#id-generation--matching-strategy) section above.

```rust
#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(track_files)]
pub struct TrackFile {
    pub id: Uuid,
    pub recording_id: Uuid,
    /// SHA-256 hash of the binary contents (32 bytes). Schema-level UNIQUE.
    pub file_hash: Vec<u8>,
    /// File size in bytes. `i64` (not `u64`) because SQLite INTEGER is signed
    /// and sqlx refuses to encode `u64` to prevent silent overflow on the top
    /// bit. 2^63 bytes ≈ 9 EB, well past any plausible single-file size.
    pub file_size: i64,
    pub format: String, // Mp3, Flac, Alac, etc.
    pub uri: String, // Path or URL to the file on the user's system. Opaque to the domain model.
}
```

### InboxItem (Unresolved Capture)
A scratchpad for items the user has captured quickly (a file path, an artist name, a free-form note) but hasn't yet matched against MusicBrainz. Each item carries its typed payload as an opaque msgpack-encoded blob so adding new variants is a code-only change — no migration required.

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize, SyncEntity)]
#[table(inbox_items)]
pub struct InboxItem {
    pub id: Uuid,
    /// msgpack-encoded `InboxValue`. Decode via `InboxItem::value()`.
    pub value: Vec<u8>,
    pub status: InboxStatus,
    /// Set when `status == Resolved`. Caller infers the target table from
    /// the decoded `InboxValue` kind.
    pub resolved_id: Option<Uuid>,
}
```

The Rust-side typed view of `value`:

```rust
pub enum InboxValue {
    File(String),    // path or URI hint
    Artist(String),  // artist name hint
    Other(String),   // free-form
}
```

Status lifecycle (stored as TEXT; closed set — unknown strings fail to decode):

```rust
pub enum InboxStatus {
    Pending,   // default; freshly captured
    Resolved,  // matched and linked via `resolved_id`
    Failed,    // tried to match, didn't find anything; kept for retry
}
```

Removing an item from view uses the standard `SyncEntity` soft-delete (`deleted = 1`). The `Failed` status is distinct: it records "we tried and didn't find a match", whereas `deleted` records "the user no longer wants to see this".
