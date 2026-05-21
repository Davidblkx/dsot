use chrono::{DateTime, Utc};
use dsot_db_sync::{
    SyncEntity,
    repo::{ListQuery, SyncEntityRepository},
};
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

struct TestRepository;

impl TestRepository {
    async fn insert<'a, E>(executor: E, entity: &ArtistSql) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        sqlx::query!(
            r#"
            INSERT INTO artists (id, name, sort_name, created, updated, deleted)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            entity.id,
            entity.name,
            entity.sort_name,
            entity.created,
            entity.updated,
            entity.deleted,
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    async fn delete<'a, E>(executor: E, id: Uuid) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE artists
            SET deleted = 1, updated = ?
            WHERE id = ?
            "#,
            now,
            id,
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    async fn restore<'a, E>(executor: E, id: Uuid) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE artists
            SET deleted = 0, updated = ?
            WHERE id = ?
            "#,
            now,
            id,
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    async fn update<'a, E>(
        executor: E,
        id: Uuid,
        updates: Vec<dsot_db_sync::model::UpdateColumnOp>,
    ) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        if updates.is_empty() {
            return Ok(());
        }

        let mut query_builder = sqlx::QueryBuilder::<sqlx::Sqlite>::new("UPDATE artists SET ");

        for (i, op) in updates.iter().enumerate() {
            if i > 0 {
                query_builder.push(", ");
            }
            query_builder.push(format!("{} = ", op.column));
            match &op.value {
                dsot_db_sync::model::UpdateValue::Null => {
                    query_builder.push_bind(None::<String>);
                }
                dsot_db_sync::model::UpdateValue::Integer(v) => {
                    query_builder.push_bind(*v);
                }
                dsot_db_sync::model::UpdateValue::Real(v) => {
                    query_builder.push_bind(*v);
                }
                dsot_db_sync::model::UpdateValue::Text(v) => {
                    query_builder.push_bind(v);
                }
                dsot_db_sync::model::UpdateValue::Blob(v) => {
                    query_builder.push_bind(v);
                }
            }
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();
        query.execute(executor).await?;

        Ok(())
    }
}

impl SyncEntityRepository for TestRepository {
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
        .fetch_optional(executor)
        .await?;

        match value {
            Some(artist) => Ok(artist),
            None => Err(dsot_db_sync::repo::RepositoryError::EntityNotFound(
                "artist",
                id.clone(),
            )),
        }
    }

    async fn list<'a, E>(
        executor: E,
        query: dsot_db_sync::repo::ListQuery,
    ) -> dsot_db_sync::repo::Result<Vec<Self::Entity>>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        let ListQuery { count, offset } = query;

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
            ORDER BY id ASC
            LIMIT ? OFFSET ?
            "#,
            count,
            offset
        )
        .fetch_all(executor)
        .await?;

        Ok(value)
    }

    async fn exec_op<'a, E>(
        executor: E,
        op: dsot_db_sync::model::SyncOperation,
    ) -> dsot_db_sync::repo::Result<()>
    where
        E: Executor<'a, Database = sqlx::Sqlite>,
    {
        match op {
            dsot_db_sync::model::SyncOperation::Create(data) => {
                let value = ArtistSql::from_bytes(&data)?;
                TestRepository::insert(executor, &value).await
            }
            dsot_db_sync::model::SyncOperation::Update(id, updates) => {
                TestRepository::update(executor, id, updates).await
            }
            dsot_db_sync::model::SyncOperation::Delete(id) => {
                TestRepository::delete(executor, id).await
            }
            dsot_db_sync::model::SyncOperation::Restore(id) => {
                TestRepository::restore(executor, id).await
            }
        }
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

    #[tokio::test]
    async fn test_repository_crud() {
        use sqlx::Connection;
        let mut conn = sqlx::SqliteConnection::connect("sqlite::memory:").await.unwrap();

        // Run migrations/create table
        sqlx::query(
            r#"
            CREATE TABLE artists (
                id BLOB PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                sort_name TEXT,
                created TEXT NOT NULL,
                updated TEXT NOT NULL,
                deleted INTEGER NOT NULL DEFAULT 0
            ) STRICT;
            "#,
        )
        .execute(&mut conn)
        .await
        .unwrap();

        let id = Uuid::now_v7();
        let artist_sql = ArtistSql {
            id,
            name: "Test Artist".to_string(),
            sort_name: Some("Artist, Test".to_string()),
            created: Utc::now(),
            updated: Utc::now(),
            deleted: false,
        };

        // Test Insert
        TestRepository::insert(&mut conn, &artist_sql).await.unwrap();

        // Test Get
        let fetched = TestRepository::get(&mut conn, &id).await.unwrap();
        assert_eq!(fetched.id, artist_sql.id);
        assert_eq!(fetched.name, artist_sql.name);
        assert_eq!(fetched.sort_name, artist_sql.sort_name);
        assert_eq!(fetched.deleted, false);

        // Test List
        let list = TestRepository::list(
            &mut conn,
            dsot_db_sync::repo::ListQuery {
                count: 10,
                offset: 0,
            },
        )
        .await
        .unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, id);

        // Test Update
        let updates = vec![dsot_db_sync::model::UpdateColumnOp {
            column: "name".to_string(),
            value: dsot_db_sync::model::UpdateValue::Text("Updated Name".to_string()),
        }];
        TestRepository::update(&mut conn, id, updates).await.unwrap();

        let fetched_updated = TestRepository::get(&mut conn, &id).await.unwrap();
        assert_eq!(fetched_updated.name, "Updated Name");

        // Test Delete
        TestRepository::delete(&mut conn, id).await.unwrap();
        let fetched_deleted = TestRepository::get(&mut conn, &id).await.unwrap();
        assert_eq!(fetched_deleted.deleted, true);

        // Test Restore
        TestRepository::restore(&mut conn, id).await.unwrap();
        let fetched_restored = TestRepository::get(&mut conn, &id).await.unwrap();
        assert_eq!(fetched_restored.deleted, false);
    }
}
