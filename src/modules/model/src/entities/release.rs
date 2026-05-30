use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl Release {
    pub fn new(id: Uuid, release_group_id: Uuid, title: String, format: String) -> Self {
        Self {
            id,
            release_group_id,
            title,
            barcode: None,
            release_date: None,
            format,
            label: None,
        }
    }
}
