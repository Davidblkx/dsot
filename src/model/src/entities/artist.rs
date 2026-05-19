use chrono::{DateTime, Utc};
use dsot_db_sync::repo::SyncEntityRepository;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub added: DateTime<Utc>,
}

#[derive(Debug, Clone, ::serde::Deserialize, ::serde::Serialize, Default, ::sqlx::FromRow)]
pub struct ArtistSQLite {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub added: DateTime<Utc>,
    pub is_deleted: bool,
}

impl From<Artist> for ArtistSQLite {
    fn from(value: Artist) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sort_name: value.sort_name,
            added: value.added,
            is_deleted: false,
        }
    }
}

impl From<ArtistSQLite> for Artist {
    fn from(value: ArtistSQLite) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sort_name: value.sort_name,
            added: value.added,
        }
    }
}

impl SyncEntityRepository for ArtistSQLite {
    type Entity = ArtistSQLite;

    fn get_table_name() -> &'static str {
        "artists"
    }

    async fn get<'a, E>(executor: E, id: &Uuid) -> dsot_db_sync::repo::Result<Self::Entity>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        let value = sqlx::query_as!(
            ArtistSQLite,
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
