use uuid::Uuid;

use music_brainz::model::artist::ArtistType;

use super::{Album, AlbumSql};

use crate::error::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ArtistV0 {
    pub id: Uuid,
    pub mbid: Option<Uuid>,
    pub name: String,
    pub sort_name: Option<String>,
    pub artist_type_id: u32,
}

crate::dsot_storage_declare_model!(Artist {
    0: ArtistV0
});

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArtistUpdateOpV0 {
    SetMbid(Option<Uuid>),
    SetName(String),
    SetSortName(Option<String>),
    SetArtistTypeId(u32),
}

crate::dsot_storage_declare_model!(ArtistUpdateOp {
    0: ArtistUpdateOpV0
});

impl Artist {
    pub fn get_artist_type(&self) -> ArtistType {
        ArtistType::from_u32(self.artist_type_id)
    }

    pub fn set_artist_type(&mut self, artist_type: ArtistType) {
        self.artist_type_id = artist_type.to_u32();
    }

    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            mbid: None,
            name: name.to_string(),
            sort_name: None,
            artist_type_id: 1,
        }
    }

    pub async fn get_aliases(
        &self,
        mut trx: sqlx::Transaction<'static, sqlx::Sqlite>
    ) -> Result<(Vec<String>, sqlx::Transaction<'static, sqlx::Sqlite>)> {
        let rows = sqlx::query!("SELECT id, artist_id, name FROM artist_aliases WHERE artist_id = ?", self.id)
            .fetch_all(&mut *trx)
            .await?;

        Ok(
            (rows.into_iter().map(|alias| alias.name).collect(), trx)
        )
    }

    pub async fn get_albums(&self, mut trx: SqlTransaction) -> SqlResult<Vec<Album>> {
        let rows = sqlx::query!("SELECT * FROM album_artists WHERE artist_id = ? AND is_main = 1", self.id)
            .fetch_all(&mut *trx)
            .await?;

        let mut albums = Vec::new();

        for row in rows {
            let id = Uuid::from_slice(&row.album_id);
            if id.is_err() {
                log::warn!("Error parsing album_id[{:?}] for artist[{:?}]", id, self.id);
                continue;
            }
            let id = id.unwrap();

            let (trx_ref, album) = AlbumSql::fetch_by_id(trx, &id).await?;
            trx = trx_ref;

            if let Some(album) = album {
                albums.push(album);
            } else {
                log::warn!("Album[{:?}] not found for artist[{:?}]", id, self.id);
            }
        }

        Ok((trx, albums))
    }
}

impl Default for Artist {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            mbid: None,
            name: String::new(),
            sort_name: None,
            artist_type_id: 0,
        }
    }
}

crate::dsot_sql_entity!(["artists"] Artist with ArtistUpdateOp {
    mbid,
    name,
    sort_name,
    artist_type_id
});

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use crate::model::entities::{artist_alias::{ArtistAlias, ArtistAliasSql}, album_artist::{AlbumArtist, AlbumArtistSql}};

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query_aliases(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let artist = Artist::new("artist");

        let (mut trx, _) = ArtistSql::insert(trx, &artist).await.unwrap();

        let aliases = vec![
            String::from("alias1"),
            String::from("alias2"),
            String::from("alias3"),
        ];

        for alias in &aliases {
            (trx, _) = ArtistAliasSql::insert(trx, &ArtistAlias::new(&artist.id, alias)).await.unwrap();
        }

        let (fetched_aliases, _) = artist.get_aliases(trx).await.unwrap();
        assert_eq!(fetched_aliases.len(), aliases.len());
        for alias in &fetched_aliases {
            assert!(aliases.contains(alias));
        }
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query_albums(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let artist = Artist::new("artist");
        let album1 = Album::new("album1", 2023);
        let album2 = Album::new("album2", 2023);

        let rel1 = AlbumArtist::new(&album1.id, &artist.id);
        let rel2 = AlbumArtist::new(&album2.id, &artist.id);

        let (trx, _) = ArtistSql::insert(trx, &artist).await.unwrap();
        let (trx, _) = AlbumSql::insert(trx, &album1).await.unwrap();
        let (trx, _) = AlbumSql::insert(trx, &album2).await.unwrap();
        let (trx, _) = AlbumArtistSql::insert(trx, &rel1).await.unwrap();
        let (trx, _) = AlbumArtistSql::insert(trx, &rel2).await.unwrap();

        let (_, albums) = artist.get_albums(trx).await.unwrap();
        assert_eq!(albums.len(), 2);
        assert!(albums.iter().any(|a| a.id == album1.id));
        assert!(albums.iter().any(|a| a.id == album2.id));
    }
}
