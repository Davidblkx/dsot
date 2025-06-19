use super::{Album, op::AlbumUpdateOp};
use crate::model::entities::album_artist::AlbumArtist;
use crate::model::entities::artist::{Artist, sql::ArtistSql};

crate::dsot_sql_entity!(["albums"] Album with AlbumUpdateOp {
    mbid: Option<uuid::Uuid>,
    title: String,
    year: Option<i16>
});

impl Album {
    /// Fetches the artists associated with this album.
    pub async fn get_artists(&self, mut trx: SqlTransaction) -> SqlResult<Vec<Artist>> {
        let rels = sqlx::query_as::<_, AlbumArtist>(
            "SELECT * FROM album_artists WHERE album_id = ? AND is_main = 1",
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
                log::warn!(
                    "Artist[{:?}] not found for album[{:?}]",
                    rel.artist_id,
                    self.id
                );
            }

            trx = trx_in;
        }

        Ok((trx, artists))
    }
}

#[cfg(test)]
mod tests {
    use super::super::sql::AlbumSql;
    use super::*;
    use crate::model::DsotEntity;
    use crate::model::entities::album_artist::{AlbumArtist, sql::AlbumArtistSql};
    use crate::model::entities::artist::{Artist, sql::ArtistSql};
    use crate::storage::SqlOperation;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query_artists(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let album = Album::new("album", 2023);
        let art1 = Artist::new("artist1");
        let art2 = Artist::new("artist2");

        let (trx, _) = AlbumSql::insert(trx, &album).await.unwrap();
        let (trx, _) = ArtistSql::insert(trx, &art1).await.unwrap();
        let (trx, _) = ArtistSql::insert(trx, &art2).await.unwrap();

        let (trx, _) = AlbumArtistSql::insert(trx, &AlbumArtist::new(&album.id, &art1.id))
            .await
            .unwrap();
        let (trx, _) = AlbumArtistSql::insert(trx, &AlbumArtist::new(&album.id, &art2.id))
            .await
            .unwrap();

        let (_, fetched_artists) = album.get_artists(trx).await.unwrap();

        assert!(
            fetched_artists.len() == 2,
            "Expected 2 artists, found {}",
            fetched_artists.len()
        );
        assert!(
            fetched_artists.iter().any(|a| a.id == art1.id),
            "Artist1 not found in fetched artists"
        );
        assert!(
            fetched_artists.iter().any(|a| a.id == art2.id),
            "Artist2 not found in fetched artists"
        );
    }

    #[test]
    fn create_valid_sql_insert_operation() {
        let album = Album {
            id: uuid::Uuid::now_v7(),
            mbid: Some(uuid::Uuid::now_v7()),
            title: "Test Album".to_string(),
            year: Some(2023),
        };
        let op = album.sql_operation().create().unwrap();
        match op {
            SqlOperation::Create { data, entity, id } => {
                assert_eq!(id, album.id);
                assert_eq!(entity, DsotEntity::Album.get_id());
                let deserialized_album = Album::deserialize(&data).unwrap();
                assert_eq!(deserialized_album.id, album.id);
                assert_eq!(deserialized_album.mbid, album.mbid);
                assert_eq!(deserialized_album.title, album.title);
                assert_eq!(deserialized_album.year, album.year);
            }
            _ => panic!("Expected a create operation"),
        }
    }

    #[test]
    fn create_valid_sql_delete_operation() {
        let album = Album {
            id: uuid::Uuid::now_v7(),
            mbid: Some(uuid::Uuid::now_v7()),
            title: "Test Album".to_string(),
            year: Some(2023),
        };
        let op = album.sql_operation().delete();
        match op {
            SqlOperation::Delete { id, entity } => {
                assert_eq!(id, album.id);
                assert_eq!(entity, DsotEntity::Album.get_id());
            }
            _ => panic!("Expected a delete operation"),
        }
    }

    #[test]
    fn create_valid_sql_update_operation() {
        let album = Album {
            id: uuid::Uuid::now_v7(),
            mbid: Some(uuid::Uuid::now_v7()),
            title: "Test Album".to_string(),
            year: Some(2023),
        };

        let new_mbid = Some(uuid::Uuid::now_v7());
        let op_mbid = album.sql_operation().update_mbid(new_mbid).unwrap();
        match op_mbid {
            SqlOperation::Update { id, action, entity } => {
                assert_eq!(id, album.id);
                assert_eq!(entity, DsotEntity::Album.get_id());
                let deserialized_op = AlbumUpdateOp::deserialize(&action).unwrap();
                if let AlbumUpdateOp::SetMbid(mbid) = deserialized_op {
                    assert_eq!(mbid, new_mbid);
                } else {
                    panic!("Expected SetMbid operation");
                }
            }
            _ => panic!("Expected an update operation"),
        }

        let new_title = "Updated Album Title".to_string();
        let op_title = album
            .sql_operation()
            .update_title(new_title.clone())
            .unwrap();
        match op_title {
            SqlOperation::Update { id, action, entity } => {
                assert_eq!(id, album.id);
                assert_eq!(entity, DsotEntity::Album.get_id());
                let deserialized_op = AlbumUpdateOp::deserialize(&action).unwrap();
                if let AlbumUpdateOp::SetTitle(title) = deserialized_op {
                    assert_eq!(title, new_title);
                } else {
                    panic!("Expected SetTitle operation");
                }
            }
            _ => panic!("Expected an update operation"),
        }

        let new_year = Some(2024);
        let op_year = album.sql_operation().update_year(new_year).unwrap();
        match op_year {
            SqlOperation::Update { id, action, entity } => {
                assert_eq!(id, album.id);
                assert_eq!(entity, DsotEntity::Album.get_id());
                let deserialized_op = AlbumUpdateOp::deserialize(&action).unwrap();
                if let AlbumUpdateOp::SetYear(year) = deserialized_op {
                    assert_eq!(year, new_year);
                } else {
                    panic!("Expected SetYear operation");
                }
            }
            _ => panic!("Expected an update operation"),
        }
    }
}
