use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct WorkV0 {
    pub id: Uuid,
    pub mbid: Option<Uuid>,
    pub title: String,
    pub kind: Option<String>,
    pub language: Option<String>,
    pub disambiguation: Option<String>,
}

crate::dsot_storage_declare_model!(
    Work {
        0: WorkV0
    }
    "
    Match the music brainz entity Work,
    It represents a musical work such as a song or composition.
    It is identified by a unique ID and may have an associated MusicBrainz ID (mbid).
    "
);
