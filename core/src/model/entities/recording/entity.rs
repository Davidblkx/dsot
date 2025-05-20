use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct RecordingV0 {
    /// Internal ID for the recording.
    pub id: Uuid,
    /// MusicBrainz ID for the recording.
    pub mbid: Option<Uuid>,
    /// Title of the recording.
    pub title: String,
    /// Length of the recording in milliseconds.
    pub length: Option<u32>,
    /// isrc (International Standard Recording Code) for the recording.
    pub isrc: Option<String>,
    /// The ID of the related work (song).
    pub work_id: Option<Uuid>,
    /// Year of first release of the recording.
    pub year: Option<i32>,
    /// Description of the recording used for disambiguation.
    pub disambiguation: Option<String>
}

crate::dsot_storage_declare_model!(
    Recording {
        0: RecordingV0
    }
    "
    Represents a specific version of a song, including details like length and date.

    Each recording can be associated with multiple tracks

    This maps to a musicbrainz recording using the mbid field.
    "
);
