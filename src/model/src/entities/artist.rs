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
                    created AS "created: DateTime<Utc>",
                    updated AS "updated: DateTime<Utc>",
                    deleted AS "deleted: bool"
                FROM artists
                WHERE id = $1
                "#,
            id
        )
        .fetch_one(executor)
        .await?;

        Ok(value)
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
    use dsot_db_sync::SyncEntity;

    use super::*;

    #[test]
    fn can_map_to_sql() {
        let input = Artist {
            id: Uuid::now_v7(),
            name: "Artist".into(),
            sort_name: Some("Artist".into()),
        };

        let sql: ArtistSql = input.clone().into();

        assert_eq!(sql.id, input.id);
        assert_eq!(sql.name, input.name);
        assert_eq!(sql.sort_name, input.sort_name);
        assert_eq!(sql.deleted, false);
    }

    #[test]
    fn can_map_to_src() {
        let input = ArtistSql {
            id: Uuid::now_v7(),
            name: "Artist".into(),
            sort_name: Some("Artist".into()),
            created: Utc::now(),
            deleted: true,
            updated: Utc::now(),
        };

        let artist: Artist = input.clone().into();

        assert_eq!(artist.id, input.id);
        assert_eq!(artist.name, input.name);
        assert_eq!(artist.sort_name, input.sort_name);
    }

    #[test]
    fn can_detect_changes() {
        let id = Uuid::now_v7();
        let a1 = ArtistSql {
            id,
            name: "n1".to_string(),
            sort_name: Some("n2".to_string()),
            ..ArtistSql::default()
        };
        let a2 = ArtistSql {
            sort_name: Some("n3".to_string()),
            ..a1.clone()
        };

        let changes = match a2.op_update(&a1).unwrap() {
            dsot_db_sync::model::SyncOperation::Update(_, list) => list,
            _ => panic!("Should be update"),
        };

        assert_eq!(2, changes.len());
        assert_eq!(
            changes[0].clone(),
            dsot_db_sync::model::UpdateColumnOp {
                column: "sort_name".to_string(),
                value: dsot_db_sync::model::UpdateValue::Text("n3".to_string()),
            }
        );
        assert_eq!(changes[1].column, "updated".to_string());
    }
}
