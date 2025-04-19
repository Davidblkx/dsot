use uuid::Uuid;

use crate::storage::{BinModel, SqlEntity};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AlbumArtistV0 {
    pub id: Uuid,
    pub album_id: Uuid,
    pub artist_id: Uuid,
    pub is_main: bool,
}

crate::dsot_storage_declare_model!(AlbumArtist {
    0: AlbumArtistV0
});

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AlbumArtistUpdateOpV0 {
    SetAlbumId(Uuid),
    SetArtistId(Uuid),
    SetIsMain(bool),
}

crate::dsot_storage_declare_model!(AlbumArtistUpdateOp {
    0: AlbumArtistUpdateOpV0
});

impl AlbumArtist {
    pub fn new(album_id: &Uuid, artist_id: &Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            album_id: *album_id,
            artist_id: *artist_id,
            is_main: true,
        }
    }
}

crate::dsot_sql_entity!(["album_artists"] AlbumArtist with AlbumArtistUpdateOp {
    album_id,
    artist_id,
    is_main
});
