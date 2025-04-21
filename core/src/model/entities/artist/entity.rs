use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ArtistV0 {
    /// Internal ID for the artist.
    pub id: Uuid,
    /// MusicBrainz ID for the artist.
    pub mbid: Option<Uuid>,
    /// Name of the artist.
    pub name: String,
    /// Sort name of the artist for sorting purposes.
    pub sort_name: Option<String>,
    /// Type of the artist, e.g., person, group, etc.
    /// It can be mapped to a type in the musicbrainz database.
    pub artist_type_id: u32,
}

crate::dsot_storage_declare_model!(
    Artist {
        0: ArtistV0
    } "
    Represents an person or group of people who work in a track, album, or release

    It maps to the `artists` table in the database. And relates to musicbrainz's by the property `mbid`.
    "
);
