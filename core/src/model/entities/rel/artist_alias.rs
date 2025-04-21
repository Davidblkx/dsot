use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ArtistAliasV0 {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub name: String
}

crate::dsot_storage_declare_model!(ArtistAlias {
    0: ArtistAliasV0
});

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArtistAliasUpdateOpV0 {
    SetName(String),
    SetArtistId(Uuid)
}

crate::dsot_storage_declare_model!(ArtistAliasUpdateOp {
    0: ArtistAliasUpdateOpV0
});

impl ArtistAlias {
    pub fn new(artist_id: &Uuid, name: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            artist_id: *artist_id,
            name: name.to_string()
        }
    }
}

crate::dsot_sql_entity!(["artist_aliases"] ArtistAlias with ArtistAliasUpdateOp {
    artist_id,
    name
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::entities::{Artist, sql::ArtistSql};
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_do_sql_crud_operations(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();
        let artist = Artist::new("Test Artist");
        let alias = ArtistAlias::new(&artist.id, "Test Alias");

        // Insert artist
        let (trx, _) = ArtistSql::insert(trx, &artist).await.unwrap();

        // Insert alias
        let (trx, _) = ArtistAliasSql::insert(trx, &alias).await.unwrap();

        // Fetch alias
        let (trx, result) = ArtistAliasSql::fetch_by_id(trx, &alias.id).await.unwrap();
        let fetched_alias: ArtistAlias = result.unwrap();
        assert_eq!(fetched_alias.id, alias.id);
        assert_eq!(fetched_alias.artist_id, alias.artist_id);
        assert_eq!(fetched_alias.name, alias.name);

        // Update alias
        let (trx, _) = ArtistAliasSql::update(
            trx,
            &alias.id,
            &ArtistAliasUpdateOp::SetName("Updated Alias".to_string())
        ).await.unwrap();

        // Fetch updated alias
        let result = ArtistAliasSql::fetch_by_id(trx, &alias.id).await.unwrap();
        let trx = result.0;
        let updated_alias: ArtistAlias = result.1.unwrap();
        assert_eq!(updated_alias.id, alias.id);
        assert_eq!(updated_alias.artist_id, alias.artist_id);
        assert_eq!(updated_alias.name, "Updated Alias");

        // Delete artist
        let (trx, _) = ArtistSql::delete(trx, &artist.id).await.unwrap();

        // Check alias is deleted
        let result = ArtistAliasSql::fetch_by_id(trx, &alias.id).await.unwrap();
        assert!(result.1.is_none());
    }
}
