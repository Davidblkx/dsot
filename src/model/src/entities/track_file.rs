use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(track_files)]
pub struct TrackFile {
    pub id: Uuid,
    pub recording_id: Uuid,
    /// SHA-256 hash of the binary contents (32 bytes). Stored as a BLOB
    /// with a UNIQUE constraint at the schema level so the same physical
    /// file indexed on multiple devices converges to a single row.
    pub file_hash: Vec<u8>,
    /// File size in bytes. Stored as `i64` because SQLite INTEGER is signed
    /// and sqlx refuses to encode `u64` for sqlite to prevent silent overflow.
    /// 2^63 bytes is ~9 EB, well past any plausible single-file size.
    pub file_size: i64,
    pub format: String,
}

impl TrackFile {
    pub fn new(
        id: Uuid,
        recording_id: Uuid,
        file_hash: Vec<u8>,
        file_size: i64,
        format: String,
    ) -> Self {
        Self {
            id,
            recording_id,
            file_hash,
            file_size,
            format,
        }
    }
}
