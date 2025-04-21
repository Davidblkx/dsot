use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AlbumV0 {
    /// Internal ID for the album.
    pub id: Uuid,
    /// MusicBrainz ID for the release_group.
    pub mbid: Option<Uuid>,
    /// Title of the album.
    pub title: String,
    /// Year of the first release.
    pub year: i16,
}

crate::dsot_storage_declare_model!(
    Album {
        0: AlbumV0
    }
    "
    Represents a group of songs released together, typically by a single artist.

    Each album can have multiple releases, which are different versions of the same album, such as vinyl, CD, deluxe, etc.

    This maps to a musicbrainz release group using the mbid field.
    "
);
