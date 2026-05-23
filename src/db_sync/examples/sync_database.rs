use dsot_db_sync::{DsotDatabase, RepositoryRegistry};
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
}

#[tokio::main]
async fn main() {
    let sql1 = sqlx::sqlite::SqlitePool::connect("sqlite::memory:?cache=shared")
        .await
        .unwrap();

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
    .execute(&sql2)
    .await
    .unwrap();

    let jrn2 = Database::builder()
        .create_with_backend(InMemoryBackend::new())
        .unwrap();

    let db1 = DsotDatabase::new(jrn1, sql1);
    let db2 = DsotDatabase::new(jrn2, sql2);

    let mut e1: MyEntitySql = MyEntity {
        id: Uuid::now_v7(),
        name: "ent1".to_string(),
        sort_name: None,
    }
    .into();

    let mut e2: MyEntitySql = MyEntity {
        id: Uuid::now_v7(),
        name: "ent2".to_string(),
        sort_name: Some("sda".to_string()),
    }
    .into();

    db1.insert::<MyEntitySqlRepository>(&e1).await.unwrap();
    e1.sort_name = Some("potato".to_string());
    db1.update::<MyEntitySqlRepository>(&e1).await.unwrap();

    db2.insert::<MyEntitySqlRepository>(&e2).await.unwrap();
    e2.sort_name = Some("tomato".to_string());
    db2.update::<MyEntitySqlRepository>(&e2).await.unwrap();

    sync_dbs(&db1, &db2).await;

    e2.sort_name = Some("tomatossss".to_string());
    db2.update::<MyEntitySqlRepository>(&e2).await.unwrap();

    sync_dbs(&db2, &db1).await;

    let items1 = db1.list::<MyEntitySqlRepository>(10, 0).await.unwrap();
    let items2 = db2.list::<MyEntitySqlRepository>(10, 0).await.unwrap();

    println!("Items in db1");
    for i in items1 {
        println!("{:?}", i);
    }

    println!("Items in db2");
    for i in items2 {
        println!("{:?}", i);
    }
}

async fn sync_dbs(db1: &DsotDatabase, db2: &DsotDatabase) {
    let keys1 = db1.get_journal_keys().unwrap();

    let miss_keys_in_2 = db2.get_keys_not_in_journal(keys1.as_slice()).unwrap();
    let miss_entries_in_1 = db2
        .get_journal_entries_not_in_array(keys1.as_slice())
        .unwrap();

    let miss_entries_in_2 = db1
        .get_journal_entries_in_array(miss_keys_in_2.as_slice())
        .unwrap();

    for e in miss_entries_in_1 {
        RepositoryRegistry::instance()
            .apply_journal(&db1, e.as_slice())
            .await
            .unwrap();
    }

    for e in miss_entries_in_2 {
        RepositoryRegistry::instance()
            .apply_journal(&db2, e.as_slice())
            .await
            .unwrap();
    }
}
