use chrono::{DateTime, Utc};
use dsot_db_sync::model::{SyncOperation, UpdateColumnOp, UpdateValue};
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

impl dsot_db_sync::SyncEntity for Artist {
    type Entity = Artist;

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn op_create(&self) -> dsot_db_sync::dser::Result<dsot_db_sync::model::SyncOperation> {
        todo!()
    }

    fn op_delete(&self) -> dsot_db_sync::model::SyncOperation {
        todo!()
    }

    fn op_restore(&self) -> dsot_db_sync::model::SyncOperation {
        dsot_db_sync::model::SyncOperation::Restore(self.id)
    }

    fn op_update(&self, prev: &Self::Entity) -> Option<dsot_db_sync::model::SyncOperation> {
        if self.id != prev.id {
            return None;
        }

        let mut list: Vec<dsot_db_sync::model::UpdateColumnOp> = Vec::new();

        match UpdateValue::get_if_diff(&prev.name, &self.name) {
            Some(value) => list.push(UpdateColumnOp {
                column: "name".to_string(),
                value,
            }),
            None => {}
        };

        match UpdateValue::get_if_diff(&prev.sort_name, &self.sort_name) {
            Some(value) => list.push(UpdateColumnOp {
                column: "sort_name".to_string(),
                value,
            }),
            None => {}
        };

        if list.len() > 0 {
            Some(SyncOperation::Update(self.id, list))
        } else {
            None
        }
    }
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
