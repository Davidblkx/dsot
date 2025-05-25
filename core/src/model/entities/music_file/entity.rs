use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct MusicFileV0 {
    pub id: Uuid,
    /// Path relative to the storage root
    /// e.g. "album/song.mp3"
    pub path: String,
    /// The storage device where this file is located
    pub storage_id: Uuid,
    /// The ID of the recording this file is associated with, if any
    pub recording_id: Option<Uuid>,
    /// Size in bytes of the file
    pub size: u32,
    /// The format of the file, represented as a u32
    /// e.g. 1 for MP3, 2 for FLAC, etc.
    pub format: u32,
    /// Defines if the user wants a better version of this file
    pub need_better: bool,
    /// Chroma fingerprint of the file, if available
    pub chromaprint: Option<String>
}

#[derive(Debug)]
pub enum MusicFileFormat {
    Unknown,
    MP3,
    FLAC,
    WAV,
    OGG,
    AAC,
    WMA,
}

crate::dsot_storage_declare_model!(
    MusicFile {
        0: MusicFileV0
    }
    "
    Represents a physical music file in the system.
    It may not exist in the current device, but may exist in another instance.
    "
);
