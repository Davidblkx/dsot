## Component: Storage & Domain Model (`music_storage`)

### ID Generation & Sync Strategy

* **Matched Media:** The primary key `id` fields for `Artist`, `ReleaseGroup`, `Release`, and `Recording` store the official MusicBrainz UUID. When two devices independently look up and add the same album, their local databases will naturally merge the metadata on sync because the IDs match perfectly.
* **Unmatched/Local Media:** If metadata is parsed from local file tags and cannot be resolved via MusicBrainz, the device generates a random `Uuid::new_v4()`. To prevent collisions when syncing unmatched media, the sync engine treats IDs without a verified remote flag as device-specific until reconciled.

---

## 1. Updated Entity Schema & Rust Representations

### Core Metadata Engine

#### Artist

```rust
pub struct Artist {
    /// MusicBrainz Artist ID if matched; otherwise a random UUIDv4.
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub is_matched: bool, // True if 'id' is a verified MBID
}

```

#### ReleaseGroup (The Abstract Album)

```rust
pub struct ReleaseGroup {
    /// MusicBrainz Release Group ID if matched; otherwise a random UUIDv4.
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub primary_type: ReleaseGroupType,
    pub is_matched: bool,
}

```

#### Release (The Specific Copy/Pressing)

```rust
pub struct Release {
    /// MusicBrainz Release ID if matched; otherwise a random UUIDv4.
    pub id: Uuid,
    pub release_group_id: Uuid,
    pub title: String,
    pub barcode: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub format: MediaFormat,
    pub label: Option<String>,
    pub is_matched: bool,
}

```

#### Recording (The Audio Mix)

```rust
pub struct Recording {
    /// MusicBrainz Recording ID if matched; otherwise a random UUIDv4.
    pub id: Uuid,
    pub title: String,
    pub duration_ms: u32,
    pub isrc: Option<String>,
    pub is_matched: bool,
}

```

#### Track (The Link)

```rust
pub struct Track {
    /// MusicBrainz Track ID if matched; otherwise a random UUIDv4.
    /// Note: MusicBrainz assigns unique IDs to tracks distinct from Recording IDs.
    pub id: Uuid,
    pub release_id: Uuid,
    pub recording_id: Uuid,
    pub position: u32,     // Track number on the disc
    pub disc_number: u32,
    pub title: String,
}

```

---

## 2. Binary Management & Sync Architecture

Binaries cannot use MBIDs because MusicBrainz does not track your personal files, compressions, or transcodes. `TrackFile` entries always use random UUIDs or a hash of the file data.

```rust
pub struct TrackFile {
    pub id: Uuid,            // Internal random UUIDv4
    pub recording_id: Uuid,  // Links back to the Recording (MBID or Local)
    pub file_hash: [u8; 32], // SHA-256 content hash (Primary key for binary deduping)
    pub file_size: u64,
    pub format: AudioFormat,
}

```

---

## 3. Storage Layer Pragmatics & Sync Logic

Using MBIDs changes how your repository handles inserts. Instead of a naive `INSERT`, your database needs `UPSERT` behavior to allow incoming sync data to enrich existing records without destroying user-specific modifications (like play counts or local paths).

```rust
#[async_trait::async_trait]
pub trait MusicRepository: Send + Sync {
    /// Upserts an artist. If the MBID already exists, updates empty fields
    /// but preserves local metadata or custom relations.
    async fn upsert_artist(&self, artist: &Artist) -> Result<()>;

    /// Resolves whether a specific MBID already exists in the local database
    /// to avoid downloading duplicate metadata payloads.
    async fn has_mbid(&self, entity_type: EntityType, id: Uuid) -> Result<bool>;
}

pub enum EntityType {
    Artist,
    ReleaseGroup,
    Release,
    Recording,
}

```

### Edge Case Handling: Upgrading Local Metadata to MBID

If a user imports an unmatched local folder (generating local random UUIDs) and later hits "Match via MusicBrainz," the system must execute a merge:

1. Fetch the MBID data.
2. Insert the new records using the MBID keys.
3. Point any existing `TrackFile` records from the old local `recording_id` to the new MBID `recording_id`.
4. Delete the orphaned local metadata records.
