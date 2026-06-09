use dsot_db_sync::DsotDatabase;
use dsot_derive::SyncEntity;
use redb::{Database, backends::InMemoryBackend};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(artists)]
pub struct MyEntity {
    pub id: Uuid,
    pub name: String,
    pub sort_name: Option<String>,
    pub aliases: sqlx::types::Json<Vec<String>>,
}

#[tokio::main]
async fn main() {
    println!("Starting");

    let sql1 = sqlx::sqlite::SqlitePool::connect("sqlite::memory:?cache=shared")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE artists (
            id BLOB PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            sort_name TEXT,
            aliases TEXT NOT NULL DEFAULT '[]',
            created TEXT NOT NULL,
            updated TEXT NOT NULL,
            deleted INTEGER NOT NULL DEFAULT 0
        ) STRICT;

        -- Index to optimize syncing lookups and sorting
        CREATE INDEX idx_artists_sync ON artists (deleted, updated, created);

        -- FTS5 virtual table for full-text search on artists
        CREATE VIRTUAL TABLE artists_fts USING fts5(
            id UNINDEXED,
            name,
            sort_name,
            aliases,
        );

        -- Triggers to keep artists_fts in sync with artists
        CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
            INSERT INTO artists_fts(id, name, sort_name, aliases)
            VALUES (
                new.id,
                new.name,
                new.sort_name,
                (SELECT group_concat(value, ' ') FROM json_each(new.aliases))
            );
        END;

        CREATE TRIGGER artists_after_delete AFTER DELETE ON artists BEGIN
            DELETE FROM artists_fts WHERE id = old.id;
        END;

        CREATE TRIGGER artists_after_update AFTER UPDATE ON artists BEGIN
            DELETE FROM artists_fts WHERE id = old.id;
            INSERT INTO artists_fts(id, name, sort_name, aliases)
            VALUES (
                new.id,
                new.name,
                new.sort_name,
                (SELECT group_concat(value, ' ') FROM json_each(new.aliases))
            );
        END;

        "#,
    )
    .execute(&sql1)
    .await
    .unwrap();

    let jrn1 = Database::builder()
        .create_with_backend(InMemoryBackend::new())
        .unwrap();

    let sql2 = sqlx::sqlite::SqlitePool::connect("sqlite::memory:?cache=shared")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE artists (
            id BLOB PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            sort_name TEXT,
            aliases TEXT NOT NULL DEFAULT '[]',
            created TEXT NOT NULL,
            updated TEXT NOT NULL,
            deleted INTEGER NOT NULL DEFAULT 0
        ) STRICT;

        -- Index to optimize syncing lookups and sorting
        CREATE INDEX idx_artists_sync ON artists (deleted, updated, created);

        -- FTS5 virtual table for full-text search on artists
        CREATE VIRTUAL TABLE artists_fts USING fts5(
            id UNINDEXED,
            name,
            sort_name,
            aliases,
        );

        -- Triggers to keep artists_fts in sync with artists
        CREATE TRIGGER artists_after_insert AFTER INSERT ON artists BEGIN
            INSERT INTO artists_fts(id, name, sort_name, aliases)
            VALUES (
                new.id,
                new.name,
                new.sort_name,
                (SELECT group_concat(value, ' ') FROM json_each(new.aliases))
            );
        END;

        CREATE TRIGGER artists_after_delete AFTER DELETE ON artists BEGIN
            DELETE FROM artists_fts WHERE id = old.id;
        END;

        CREATE TRIGGER artists_after_update AFTER UPDATE ON artists BEGIN
            DELETE FROM artists_fts WHERE id = old.id;
            INSERT INTO artists_fts(id, name, sort_name, aliases)
            VALUES (
                new.id,
                new.name,
                new.sort_name,
                (SELECT group_concat(value, ' ') FROM json_each(new.aliases))
            );
        END;

        "#,
    )
    .execute(&sql2)
    .await
    .unwrap();

    let jrn2 = Database::builder()
        .create_with_backend(InMemoryBackend::new())
        .unwrap();

    let db1 = DsotDatabase::new(jrn1, sql1);
    let db2 = DsotDatabase::new(jrn2, sql2);

    println!("DB Created!");

    let mut e1: MyEntitySql = MyEntity {
        id: Uuid::now_v7(),
        name: "ent1".to_string(),
        sort_name: None,
        aliases: sqlx::types::Json(vec!["bananas".to_string()]),
    }
    .into();

    let mut e2: MyEntitySql = MyEntity {
        id: Uuid::now_v7(),
        name: "ent2".to_string(),
        sort_name: Some("sda".to_string()),
        aliases: sqlx::types::Json(vec![]),
    }
    .into();

    db1.insert::<MyEntitySqlRepository>(&e1).await.unwrap();
    e1.sort_name = Some("potato".to_string());
    db1.update::<MyEntitySqlRepository>(&e1).await.unwrap();

    db2.insert::<MyEntitySqlRepository>(&e2).await.unwrap();
    e2.sort_name = Some("tomato".to_string());
    db2.update::<MyEntitySqlRepository>(&e2).await.unwrap();

    println!("Syncing db2 to db1...");
    let db2_sync_handler = db2.create_sync_handler().await.unwrap();
    println!("Created handler, syncing...");
    db1.start_remote_sync(&db2_sync_handler).await.unwrap();
    println!("db1 completed, commit 2...");
    db2_sync_handler.commit().await.unwrap();
    println!(
        "Sync completed, db2 hash: {:?}",
        db2.generate_sync_hash().unwrap()
    );

    e2.sort_name = Some("tomatossss".to_string());
    db2.update::<MyEntitySqlRepository>(&e2).await.unwrap();

    let db2_sync_handler = db2.create_sync_handler().await.unwrap();
    db1.start_remote_sync(&db2_sync_handler).await.unwrap();
    db2_sync_handler.commit().await.unwrap();

    let items1 = db1.list::<MyEntitySqlRepository>(10, 0).await.unwrap();
    let items2 = db2.list::<MyEntitySqlRepository>(10, 0).await.unwrap();

    println!("Items in db1: {:?}", db1.generate_sync_hash().unwrap());
    for i in items1 {
        println!("{:?}", i);
    }

    println!("Items in db2: {:?}", db2.generate_sync_hash().unwrap());
    for i in items2 {
        println!("{:?}", i);
    }
}
