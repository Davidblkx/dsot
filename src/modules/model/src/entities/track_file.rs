use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(track_files)]
pub struct TrackFile {
    pub id: Uuid,
    pub recording_id: Uuid,
    pub file_hash: Vec<u8>,
    pub file_size: i64,
    pub format: String,
    pub uri: String,
}

impl TrackFile {
    pub fn new(
        id: Uuid,
        recording_id: Uuid,
        file_hash: Vec<u8>,
        file_size: i64,
        format: String,
        uri: String,
    ) -> Self {
        Self {
            id,
            recording_id,
            file_hash,
            file_size,
            format,
            uri,
        }
    }
}
