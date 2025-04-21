use super::{Album, op::AlbumUpdateOp};
use crate::model::entities::artist::{Artist, sql::ArtistSql};
use crate::model::entities::rel::AlbumArtist;

crate::dsot_sql_entity!(["albums"] Album with AlbumUpdateOp {
    mbid,
    title,
    year
});

impl Album {
    /// Fetches the artists associated with this album.
    pub async fn get_artists(
        &self,
        mut trx: SqlTransaction,
    ) -> SqlResult<Vec<Artist>> {
        let rels = sqlx::query_as::<_, AlbumArtist>(
                "SELECT * FROM album_artists WHERE album_id = ? AND is_main = 1"
            )
            .bind(self.id)
            .fetch_all(&mut *trx)
            .await?;

        let mut artists = Vec::new();

        for rel in rels {
            let (trx_in, artist) = ArtistSql::fetch_by_id(trx, &rel.artist_id).await?;
            if let Some(artist) = artist {
                artists.push(artist);
            } else {
                log::warn!("Artist[{:?}] not found for album[{:?}]", rel.artist_id, self.id);
            }

            trx = trx_in;
        }

        Ok((trx, artists))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::sql::AlbumSql;
    use crate::model::entities::artist::{Artist, sql::ArtistSql};
    use crate::model::entities::rel::{AlbumArtist, AlbumArtistSql};

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query_artists(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let album = Album::new("album", 2023);
        let art1 = Artist::new("artist1");
        let art2 = Artist::new("artist2");

        let (trx, _) = AlbumSql::insert(trx, &album).await.unwrap();
        let (trx, _) = ArtistSql::insert(trx, &art1).await.unwrap();
        let (trx, _) = ArtistSql::insert(trx, &art2).await.unwrap();

        let (trx, _) = AlbumArtistSql::insert(trx, &AlbumArtist::new(&album.id, &art1.id)).await.unwrap();
        let (trx, _) = AlbumArtistSql::insert(trx, &AlbumArtist::new(&album.id, &art2.id)).await.unwrap();

        let (_, fetched_artists) = album.get_artists(trx).await.unwrap();

        assert!(fetched_artists.len() == 2, "Expected 2 artists, found {}", fetched_artists.len());
        assert!(fetched_artists.iter().any(|a| a.id == art1.id), "Artist1 not found in fetched artists");
        assert!(fetched_artists.iter().any(|a| a.id == art2.id), "Artist2 not found in fetched artists");
    }
}
