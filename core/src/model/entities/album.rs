use uuid::Uuid;

use super::{Artist, ArtistSql};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AlbumV0 {
    pub id: Uuid,
    pub mbid: Option<Uuid>,
    pub title: String,
    pub year: i16,
}

crate::dsot_storage_declare_model!(Album {
    0: AlbumV0
});

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AlbumUpdateOpV0 {
    SetMbid(Option<Uuid>),
    SetTitle(String),
    SetYear(i16),
}

crate::dsot_storage_declare_model!(AlbumUpdateOp {
    0: AlbumUpdateOpV0
});

impl Album {
    pub fn new(title: &str, year: i16) -> Self {
        Self {
            id: Uuid::now_v7(),
            mbid: None,
            title: title.to_string(),
            year,
        }
    }

    pub async fn get_artists(
        &self,
        mut trx: SqlTransaction
    ) -> SqlResult<Vec<Artist>> {
        let rows = sqlx::query!("SELECT * FROM album_artists WHERE album_id = ? AND is_main = 1", self.id)
            .fetch_all(&mut *trx)
            .await?;

        let mut artists = Vec::new();

        for row in rows {
            let id = Uuid::from_slice(&row.artist_id);
            if id.is_err() {
                log::warn!("Error parsing artist_id[{:?}] for album[{:?}]", id, self.id);
                continue;
            }
            let id = id.unwrap();

            let (trx_ref, artist) = ArtistSql::fetch_by_id(trx, &id).await?;
            trx = trx_ref;

            if let Some(artist) = artist {
                artists.push(artist);
            } else {
                log::warn!("Artist[{:?}] not found for album[{:?}]", id, self.id);
            }
        }

        Ok((trx, artists))
    }
}

crate::dsot_sql_entity!(["albums"] Album with AlbumUpdateOp {
    mbid,
    title,
    year
});

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use crate::model::entities::album_artist::{AlbumArtist, AlbumArtistSql};

    #[sqlx::test(migrations = "../migrations")]
    async fn can_load_artists(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let album = Album::new("test_album", 2023);
        let artist1 = Artist::new("test_artist1");
        let artist2 = Artist::new("test_artist2");

        let rel1 = AlbumArtist::new(&album.id, &artist1.id);
        let rel2 = AlbumArtist::new(&album.id, &artist2.id);

        let (trx, _) = AlbumSql::insert(trx, &album).await.unwrap();
        let (trx, _) = ArtistSql::insert(trx, &artist1).await.unwrap();
        let (trx, _) = ArtistSql::insert(trx, &artist2).await.unwrap();

        let (trx, _) = AlbumArtistSql::insert(trx, &rel1).await.unwrap();
        let (trx, _) = AlbumArtistSql::insert(trx, &rel2).await.unwrap();

        let (_, artists) = album.get_artists(trx).await.unwrap();

        assert_eq!(artists.len(), 2);
        assert!(artists.iter().any(|a| a.id == artist1.id));
        assert!(artists.iter().any(|a| a.id == artist2.id));
    }
}
