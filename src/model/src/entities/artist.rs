use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(artists)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use dsot_db_sync::{SyncEntity, SyncEntityRepository};

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
        let mut conn = sqlx::SqliteConnection::connect("sqlite::memory:")
            .await
            .unwrap();

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
        ArtistSqlRepository::insert(&mut conn, &artist_sql)
            .await
            .unwrap();

        // Test Get
        let fetched = ArtistSqlRepository::get(&mut conn, id).await.unwrap();
        assert_eq!(fetched.id, artist_sql.id);
        assert_eq!(fetched.name, artist_sql.name);
        assert_eq!(fetched.sort_name, artist_sql.sort_name);
        assert_eq!(fetched.deleted, false);

        // Test List
        let list = ArtistSqlRepository::list(
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
        ArtistSqlRepository::update(&mut conn, id, updates)
            .await
            .unwrap();

        let fetched_updated = ArtistSqlRepository::get(&mut conn, id).await.unwrap();
        assert_eq!(fetched_updated.name, "Updated Name");

        // Test Delete
        ArtistSqlRepository::delete(&mut conn, id).await.unwrap();
        let fetched_deleted = ArtistSqlRepository::get(&mut conn, id).await.unwrap();
        assert_eq!(fetched_deleted.deleted, true);

        // Test Restore
        ArtistSqlRepository::restore(&mut conn, id).await.unwrap();
        let fetched_restored = ArtistSqlRepository::get(&mut conn, id).await.unwrap();
        assert_eq!(fetched_restored.deleted, false);
    }
}
