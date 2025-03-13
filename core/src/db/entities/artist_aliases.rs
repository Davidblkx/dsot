use uuid::Uuid;

crate::dsot_sql_entity!{
    ArtistAliaseV0 ["artist_aliases"] {
        id: Uuid => uuid,
        artist_id: Uuid => uuid,
        name: String => string,
    }
}

crate::dsot_storage_declare_model!(ArtistAliase {
    0: ArtistAliaseV0
});

crate::dsot_storage_use_id_uuid!(ArtistAliase, "artist_aliases");

impl ArtistAliase {
    pub fn new(artist_id: &Uuid, name: &str) -> Self {
        Self {
            id: Uuid::now_v7(),
            artist_id: artist_id.clone(),
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Artist;
    use sqlx::Executor;
    use sqlx::sqlite::SqlitePool;
    use crate::db::sql::SqlEntity;

    use crate::db::DbOperation;

    #[sqlx::test(migrations = "../migrations")]
    async fn artist_aliases_sql_crud(pool: SqlitePool) -> sqlx::Result<()> {
        // TODO: Add tests and think of a macro for sql actions
        Ok(())
    }
}
