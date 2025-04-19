use uuid::Uuid;

use music_brainz::model::artist::ArtistType;

use crate::storage::{BinModel, SqlEntity};
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
    use crate::model::entities::artist_alias::ArtistAlias;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_do_sql_crud_operations(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let artist = Artist {
            id: Uuid::now_v7(),
            mbid: Some(Uuid::now_v7()),
            name: String::from("test_name"),
            sort_name: None,
            artist_type_id: 1,
        };

        // Insert
        let trx = Artist::execute_sql_insert(trx, &artist).await.unwrap();

        // Fetch by ID
        let result = Artist::execute_sql_fetch_by_id(trx, &artist.id).await.unwrap();
        let trx = result.0;
        let fetched_artist = result.1.unwrap();
        assert_eq!(fetched_artist.id, artist.id);
        assert_eq!(fetched_artist.mbid, artist.mbid);
        assert_eq!(fetched_artist.name, artist.name);
        assert_eq!(fetched_artist.sort_name, artist.sort_name);
        assert_eq!(fetched_artist.artist_type_id, artist.artist_type_id);

        // Update Mbid
        let trx = Artist::execute_sql_update(
            trx,
            &artist.id,
            &ArtistUpdateOp::SetMbid(None),
        )
        .await
        .unwrap();

        // Update Name
        let trx = Artist::execute_sql_update(
            trx,
            &artist.id,
            &ArtistUpdateOp::SetName(String::from("new_name")),
        )
        .await
        .unwrap();

        // Update Sort Name
        let trx = Artist::execute_sql_update(
            trx,
            &artist.id,
            &ArtistUpdateOp::SetSortName(Some(String::from("new_sort_name"))),
        )
        .await
        .unwrap();

        // Update Artist Type ID
        let trx = Artist::execute_sql_update(
            trx,
            &artist.id,
            &ArtistUpdateOp::SetArtistTypeId(2),
        )
        .await
        .unwrap();

        // Fetch by ID again to check the updates
        let result = Artist::execute_sql_fetch_by_id(trx, &artist.id).await.unwrap();
        let trx = result.0;
        let fetched_artist = result.1.unwrap();
        assert_eq!(fetched_artist.mbid, None);
        assert_eq!(fetched_artist.name, "new_name");
        assert_eq!(fetched_artist.sort_name, Some("new_sort_name".to_string()));
        assert_eq!(fetched_artist.artist_type_id, 2);

        // Delete
        let trx = Artist::execute_sql_delete(trx, &artist.id).await.unwrap();

        // Fetch by ID again to check the deletion
        let result = Artist::execute_sql_fetch_by_id(trx, &artist.id).await.unwrap();
        assert!(result.1.is_none());
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query_aliases(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let artist = Artist::new("artist");

        let mut trx = Artist::execute_sql_insert(trx, &artist).await.unwrap();

        let aliases = vec![
            String::from("alias1"),
            String::from("alias2"),
            String::from("alias3"),
        ];

        for alias in &aliases {
            trx = ArtistAlias::execute_sql_insert(trx, &ArtistAlias::new(&artist.id, alias)).await.unwrap();
        }

        let (fetched_aliases, _) = artist.get_aliases(trx).await.unwrap();
        assert_eq!(fetched_aliases.len(), aliases.len());
        for alias in &fetched_aliases {
            assert!(aliases.contains(alias));
        }
    }
}
