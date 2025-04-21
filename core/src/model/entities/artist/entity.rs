use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ArtistV0 {
    pub id: Uuid,
    pub mbid: Option<Uuid>,
    pub name: String,
    pub sort_name: Option<String>,
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
