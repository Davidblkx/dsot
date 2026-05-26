use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(tracks)]
pub struct Track {
    pub id: Uuid,
    pub release_id: Uuid,
    pub recording_id: Uuid,
    pub position: u32,
    pub disc_number: u32,
    pub title: String,
}

impl Track {
    pub fn new(
        id: Uuid,
        release_id: Uuid,
        recording_id: Uuid,
        position: u32,
        disc_number: u32,
        title: String,
    ) -> Self {
        Self {
            id,
            release_id,
            recording_id,
            position,
            disc_number,
            title,
        }
    }
}
