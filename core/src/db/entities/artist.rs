use uuid::Uuid;

use crate::db::sql::{SqlEntity, SqlValue};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ArtistV0 {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
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
        vec!["id", "name", "sort_name"]
    }

    fn values(&self) -> Vec<String> {
        vec![
            SqlValue::uuid(&self.id),
            SqlValue::string(&self.name),
            match &self.sort_name {
                Some(sort_name) => SqlValue::string(sort_name),
                None => SqlValue::null(),
            }
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
            sort_name: Some("Artist, Test".to_string()),
        };
        let op = DbOperation::create_artist(&artist).unwrap();
        let op_sql = op.generate_sql().unwrap();

        let mut conn = pool.acquire().await?;

        conn.execute(op_sql.as_str()).await?;

        conn.close().await?;

        let row = sqlx::query_as::<_, Artist>("SELECT * from artists WHERE id = ?1")
            .bind(artist.id)
            .fetch_one(&pool)
            .await?;

        assert_eq!(row.id, artist.id);
        assert_eq!(row.name, artist.name);
        assert_eq!(row.sort_name, artist.sort_name);

        Ok(())
    }
}
