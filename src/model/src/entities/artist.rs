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

            CREATE VIRTUAL TABLE artists_fts USING fts5(
                id UNINDEXED,
                name,
                sort_name
            );

            CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
                INSERT INTO artists_fts(id, name, sort_name) VALUES (new.id, new.name, new.sort_name);
            END;

            CREATE TRIGGER artists_after_delete AFTER DELETE ON artists BEGIN
                DELETE FROM artists_fts WHERE id = old.id;
            END;

            CREATE TRIGGER artists_after_update AFTER UPDATE ON artists BEGIN
                UPDATE artists_fts SET name = new.name, sort_name = new.sort_name WHERE id = old.id;
            END;
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

        // Test Search
        let search_results = ArtistSqlRepository::search(&mut conn, "Test*".to_string())
            .await
            .unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, id);
        assert_eq!(search_results[0].name, "Test Artist");

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

        // Test Search after Update
        let search_results_updated = ArtistSqlRepository::search(&mut conn, "Updated*".to_string())
            .await
            .unwrap();
        assert_eq!(search_results_updated.len(), 1);
        assert_eq!(search_results_updated[0].name, "Updated Name");

        // Test Delete
        ArtistSqlRepository::delete(&mut conn, id).await.unwrap();
        let fetched_deleted = ArtistSqlRepository::get(&mut conn, id).await.unwrap();
        assert_eq!(fetched_deleted.deleted, true);

        // Test Search after Delete (should be empty because of a.deleted = 0)
        let search_results_deleted = ArtistSqlRepository::search(&mut conn, "Updated*".to_string())
            .await
            .unwrap();
        assert_eq!(search_results_deleted.len(), 0);

        // Test Restore
        ArtistSqlRepository::restore(&mut conn, id).await.unwrap();
        let fetched_restored = ArtistSqlRepository::get(&mut conn, id).await.unwrap();
        assert_eq!(fetched_restored.deleted, false);

        // Test Search after Restore
        let search_results_restored = ArtistSqlRepository::search(&mut conn, "Updated*".to_string())
            .await
            .unwrap();
        assert_eq!(search_results_restored.len(), 1);
    }

    #[tokio::test]
    async fn test_apply_journal_duplicate_create() {
        use redb::{Database, backends::InMemoryBackend};
        use dsot_db_sync::{DsotDatabase, model::JournalEntry};

        let pool = sqlx::sqlite::SqlitePool::connect("sqlite::memory:")
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

            CREATE VIRTUAL TABLE artists_fts USING fts5(
                id UNINDEXED,
                name,
                sort_name
            );

            CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
                INSERT INTO artists_fts(id, name, sort_name) VALUES (new.id, new.name, new.sort_name);
            END;
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let jrn = Database::builder()
            .create_with_backend(InMemoryBackend::new())
            .unwrap();

        let db = DsotDatabase::new(jrn, pool);

        let artist = Artist {
            id: Uuid::now_v7(),
            name: "Duplicate Artist".to_string(),
            sort_name: Some("Artist, Duplicate".to_string()),
        };
        let artist_sql: ArtistSql = artist.clone().into();

        // 1. Insert it normally (which does a local Create operation and saves to database and journal)
        db.insert::<ArtistSqlRepository>(&artist_sql).await.unwrap();

        // 2. Build a journal entry with a duplicate Create for the same artist
        let op = artist_sql.op_create().unwrap();
        let entry = JournalEntry::new("artists", &op);

        // 3. Call apply_journal. This should NOT fail because the entity ID is already in the database!
        let journal_id = db.apply_journal::<ArtistSqlRepository>(entry).await.unwrap();

        // 4. Verify that the journal entry was saved in redb
        let keys = db.get_journal_keys().unwrap();
        assert!(keys.contains(&journal_id.to_bytes_le()));

        // 5. Verify the entity is still in SQLite and matches
        let fetched = db.get::<ArtistSqlRepository>(artist.id).await.unwrap();
        assert_eq!(fetched.name, "Duplicate Artist");
    }

    #[tokio::test]
    async fn test_apply_journals_batch() {
        use redb::{Database, backends::InMemoryBackend};
        use dsot_db_sync::{DsotDatabase, RepositoryRegistry, model::JournalEntry};

        let pool = sqlx::sqlite::SqlitePool::connect("sqlite::memory:")
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

            CREATE VIRTUAL TABLE artists_fts USING fts5(
                id UNINDEXED,
                name,
                sort_name
            );

            CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
                INSERT INTO artists_fts(id, name, sort_name) VALUES (new.id, new.name, new.sort_name);
            END;
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let jrn = Database::builder()
            .create_with_backend(InMemoryBackend::new())
            .unwrap();

        let db = DsotDatabase::new(jrn, pool);

        let artist1 = Artist {
            id: Uuid::now_v7(),
            name: "Artist One".to_string(),
            sort_name: Some("Artist, One".to_string()),
        };
        let artist_sql1: ArtistSql = artist1.clone().into();

        let artist2 = Artist {
            id: Uuid::now_v7(),
            name: "Artist Two".to_string(),
            sort_name: Some("Artist, Two".to_string()),
        };
        let artist_sql2: ArtistSql = artist2.clone().into();

        let op1 = artist_sql1.op_create().unwrap();
        let entry1 = JournalEntry::new("artists", &op1);
        let bytes1 = entry1.to_bytes().unwrap();

        let op2 = artist_sql2.op_create().unwrap();
        let entry2 = JournalEntry::new("artists", &op2);
        let bytes2 = entry2.to_bytes().unwrap();

        // Apply multiple journals in batch!
        let journal_ids = RepositoryRegistry::instance()
            .apply_journals(&db, &[bytes1, bytes2])
            .await
            .unwrap();

        assert_eq!(journal_ids.len(), 2);

        // Verify journal keys
        let keys = db.get_journal_keys().unwrap();
        assert!(keys.contains(&journal_ids[0].to_bytes_le()));
        assert!(keys.contains(&journal_ids[1].to_bytes_le()));

        // Verify entities in SQLite
        let fetched1 = db.get::<ArtistSqlRepository>(artist1.id).await.unwrap();
        assert_eq!(fetched1.name, "Artist One");

        let fetched2 = db.get::<ArtistSqlRepository>(artist2.id).await.unwrap();
        assert_eq!(fetched2.name, "Artist Two");
    }
}


