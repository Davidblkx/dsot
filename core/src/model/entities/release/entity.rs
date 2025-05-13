use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ReleaseV0 {
    /// Internal ID for the release.
    pub id: Uuid,
    /// MusicBrainz ID for the release.
    pub mbid: Option<Uuid>,
    /// Title of the release.
    pub title: String,
    /// Year of release.
    pub year: Option<i32>,
    /// Status of the release.
    pub status: Option<u32>,
    /// Country of release.
    pub country: Option<String>,
    /// Duration of the release.
    pub duration: Option<i64>,
    /// Format of the release (e.g., CD, vinyl, digital).
    pub format: Option<String>,
    /// Internal ID for the release group (album).
    pub album_id: Uuid,
}

crate::dsot_storage_declare_model!(
    Release {
        0: ReleaseV0
    }
    "
    Represents a specific version of an album, including details like format, tracklist, and release year.

    Each release is associated with a release group (album) and can have multiple formats (CD, vinyl, digital, etc.).

    This maps to a musicbrainz release using the mbid field.
    "
);
