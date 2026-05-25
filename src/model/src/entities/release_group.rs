use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(release_groups)]
pub struct ReleaseGroup {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub primary_type: String,
}

impl ReleaseGroup {
    pub fn new(id: Uuid, artist_id: Uuid, title: String, primary_type: String) -> Self {
        Self {
            id,
            artist_id,
            title,
            primary_type,
        }
    }
}
