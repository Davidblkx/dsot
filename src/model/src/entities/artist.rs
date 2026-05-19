use chrono::{DateTime, Utc};
use dsot_db_sync::repo::SyncEntityRepository;
use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub added: DateTime<Utc>,
}

impl SyncEntityRepository for ArtistSql {
    type Entity = ArtistSql;

    fn get_table_name() -> &'static str {
        "artists"
    }

    async fn get<'a, E>(executor: E, id: &Uuid) -> dsot_db_sync::repo::Result<Self::Entity>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        let value = sqlx::query_as!(
            ArtistSql,
            r#"
                SELECT
                    id AS "id: Uuid",
                    name,
                    sort_name,
                    added AS "added: DateTime<Utc>",
                    is_deleted AS "is_deleted: bool"
                FROM artists
                WHERE id = $1
                "#,
            id
        )
        .fetch_one(executor)
        .await?;

        Ok(value)
    }

    async fn insert<'a, E>(executor: E, value: &Self::Entity) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        todo!()
    }

    async fn update<'a, E>(executor: E, value: &Self::Entity) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        todo!()
    }

    async fn delete<'a, E>(executor: E, id: &Uuid) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        todo!()
    }

    async fn restore<'a, E>(executor: E, id: &Uuid) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        todo!()
    }

    async fn list<'a, E>(
        executor: E,
        query: dsot_db_sync::repo::ListQuery,
    ) -> dsot_db_sync::repo::Result<Vec<Self::Entity>>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        todo!()
    }

    async fn exec_op<'a, E>(
        executor: E,
        op: dsot_db_sync::model::SyncOperation,
    ) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_map_to_sql() {
        let input = Artist {
            id: Uuid::now_v7(),
            name: "Artist".into(),
            sort_name: Some("Artist".into()),
            added: Utc::now(),
        };

        let sql: ArtistSql = input.clone().into();

        assert_eq!(sql.id, input.id);
        assert_eq!(sql.name, input.name);
        assert_eq!(sql.sort_name, input.sort_name);
        assert_eq!(sql.added, input.added);
        assert_eq!(sql.is_deleted, false);
    }

    #[test]
    fn can_map_to_src() {
        let input = ArtistSql {
            id: Uuid::now_v7(),
            name: "Artist".into(),
            sort_name: Some("Artist".into()),
            added: Utc::now(),
            is_deleted: true,
        };

        let artist: Artist = input.clone().into();

        assert_eq!(artist.id, input.id);
        assert_eq!(artist.name, input.name);
        assert_eq!(artist.sort_name, input.sort_name);
        assert_eq!(artist.added, input.added);
    }
}
