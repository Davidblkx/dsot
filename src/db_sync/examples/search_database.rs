use dsot_db_sync::{DsotDatabase, IntoSyncEntity};
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

impl MyEntity {
    pub fn new(name: &str) -> Self {
        MyEntity {
            id: Uuid::now_v7(),
            name: name.to_string(),
            sort_name: None,
        }
    }
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

    let db = DsotDatabase::new(jrn1, sql1);

    let e1 = MyEntity::new("Tom Waits").to_sync();
    let e2 = MyEntity::new("Tom Jones").to_sync();
    let e3 = MyEntity::new("Jones and Friends").to_sync();
    let e4 = MyEntity::new("Wait for me").to_sync();
    let e5 = MyEntity::new("Pink Floyd").to_sync();

    db.insert::<MyEntitySqlRepository>(&e1).await.unwrap();
    db.insert::<MyEntitySqlRepository>(&e2).await.unwrap();
    db.insert::<MyEntitySqlRepository>(&e3).await.unwrap();
    db.insert::<MyEntitySqlRepository>(&e4).await.unwrap();
    db.insert::<MyEntitySqlRepository>(&e5).await.unwrap();

    let items = db.search::<MyEntitySqlRepository>("and*").await.unwrap();

    for item in items {
        println!("{:?}", item);
    }
}
