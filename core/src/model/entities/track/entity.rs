use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TrackV0 {
    pub id: Uuid,
    pub release_media_id: Uuid,
    pub media_index: u32,
    pub release_index: u32,
    pub track_number: u32,
    pub position: Option<String>,
    pub title: String,
    pub mbid: Option<Uuid>,
    pub recording_id: Uuid,
}

crate::dsot_storage_declare_model!(
    Track {
        0: TrackV0
    }
    "
    Represents a track in a release. It's always associated with a recording and a release_media.
    "
);
