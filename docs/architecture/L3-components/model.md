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
    Personal audio binaries use their **SHA-256 content hash** as their primary key for deduplication. This allows identical files to be deduped across the system even if they have different filenames or local paths.

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
}
```

---

## Planned Entities `[PLANNED / ROADMAP]`

The following entities represent the planned schema for media and playlist management.

### ReleaseGroup (The Abstract Album)
Represents the logical grouping of releases (e.g., "The Dark Side of the Moon" as a concept).

```rust
pub struct ReleaseGroup {
    pub id: Uuid,             // MusicBrainz Release Group ID or generated Uuid
    pub artist_id: Uuid,      // Links to Artist id
    pub title: String,
    pub primary_type: String, // Album, Single, EP, Live, etc.
}
```

### Release (The Specific Pressing)
Represents a specific physical or digital pressing of an album (e.g., the 1993 Remaster or a UK Vinyl release).

```rust
pub struct Release {
    pub id: Uuid,             // MusicBrainz Release ID or generated Uuid
    pub release_group_id: Uuid,
    pub title: String,
    pub barcode: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub format: String,       // CD, Vinyl, Digital, etc.
    pub label: Option<String>,
}
```

### Recording (The Audio Mix)
Represents a unique audio mix or master track.

```rust
pub struct Recording {
    pub id: Uuid,             // MusicBrainz Recording ID or generated Uuid
    pub title: String,
    pub duration_ms: u32,
    pub isrc: Option<String>, // International Standard Recording Code
}
```

### Track (The Album Position Link)
Links a specific `Recording` to a position on a `Release`.

```rust
pub struct Track {
    pub id: Uuid,             // MusicBrainz Track ID (distinct from Recording ID)
    pub release_id: Uuid,
    pub recording_id: Uuid,
    pub position: u32,        // Track number (e.g. 1, 2, 3)
    pub disc_number: u32,     // Multi-disc albums (e.g. 1, 2)
    pub title: String,
}
```

### TrackFile (Local Binary File)
Links a local physical audio file on the user's filesystem to a domain `Recording`.

```rust
pub struct TrackFile {
    pub id: Uuid,            // Generated Uuid
    pub recording_id: Uuid,  // Links to Recording id
    pub file_hash: [u8; 32], // SHA-256 hash of the binary file for deduping
    pub file_size: u64,
    pub format: String,      // Mp3, Flac, Alac, etc.
}
```
