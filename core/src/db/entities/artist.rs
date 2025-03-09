use uuid::Uuid;

use crate::db::sql::SqlEntity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArtistV0 {
    pub id: Uuid,
    pub name: String,
}

crate::dsot_storage_declare_model!(Artist {
    0: ArtistV0
});

crate::dsot_storage_use_id_uuid!(Artist, "artist");

impl SqlEntity for Artist {
    fn table_name() -> &'static str {
        "artists"
    }

    fn columns() -> Vec<&'static str> {
        vec!["id", "name"]
    }

    fn values(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Executor;
    use sqlx::sqlite::SqlitePool;

    use crate::db::DbOperation;

    #[sqlx::test(migrations = "../migrations")]
    async fn sql_create_entity(pool: SqlitePool) -> sqlx::Result<()> {
        let artist = Artist {
            id: Uuid::now_v7(),
            name: "Test Artist".to_string(),
        };
        let op = DbOperation::create_artist(&artist).unwrap();
        let op_sql = op.generate_sql().unwrap();

        let mut conn = pool.acquire().await?;

        conn.execute(op_sql.as_str()).await?;

        Ok(())
    }
}
