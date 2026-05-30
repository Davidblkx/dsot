use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(recordings)]
pub struct Recording {
    pub id: Uuid,
    pub title: String,
    pub duration_ms: u32,
    pub isrc: Option<String>,
}

impl Recording {
    pub fn new(id: Uuid, title: String, duration_ms: u32) -> Self {
        Self {
            id,
            title,
            duration_ms,
            isrc: None,
        }
    }

    pub fn with_isrc(mut self, isrc: String) -> Self {
        self.isrc = Some(isrc);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dsot_db_sync::{DsotDatabase, IntoSyncEntity, SyncEntity, model::JournalEntry};
    use redb::{Database, backends::InMemoryBackend};
    use sqlx::SqlitePool;

    fn setup_database(pool: SqlitePool) -> DsotDatabase {
        let journal = Database::builder()
            .create_with_backend(InMemoryBackend::new())
            .expect("create in-memory redb");
        DsotDatabase::new(journal, pool)
    }

    #[sqlx::test(migrations = "../../../migrations")]
    async fn test_recording_crud_and_search(pool: SqlitePool) {
        let db = setup_database(pool);
        let id = Uuid::now_v7();
        let recording = Recording::new(id, "Comfortably Numb".to_string(), 382000)
            .with_isrc("USSM19900142".to_string());

        // Test insert
        let sync_entity = recording.to_sync();
        db.insert::<RecordingSqlRepository>(&sync_entity)
            .await
            .unwrap();

        // Test get
        let fetched = db.get::<RecordingSqlRepository>(id).await.unwrap();
        assert_eq!(fetched.title, "Comfortably Numb");
        assert_eq!(fetched.duration_ms, 382000);
        assert_eq!(fetched.isrc, Some("USSM19900142".to_string()));

        // Test search via FTS5
        let search_results = db
            .search::<RecordingSqlRepository>("Comfortably")
            .await
            .unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, id);

        // Test search via FTS5 on isrc
        let search_results_isrc = db
            .search::<RecordingSqlRepository>("USSM19900142")
            .await
            .unwrap();
        assert_eq!(search_results_isrc.len(), 1);
        assert_eq!(search_results_isrc[0].id, id);

        // Test update
        let mut updated = fetched.clone();
        updated.title = "Comfortably Numb (2011 Remaster)".to_string();

        let op = updated.op_update(&fetched).expect("change detected");
        db.apply_journal::<RecordingSqlRepository>(JournalEntry::new("recordings", &op))
            .await
            .unwrap();

        let fetched_updated = db.get::<RecordingSqlRepository>(id).await.unwrap();
        assert_eq!(fetched_updated.title, "Comfortably Numb (2011 Remaster)");

        // Test delete
        db.delete::<RecordingSqlRepository>(id).await.unwrap();
        let fetched_deleted = db.get::<RecordingSqlRepository>(id).await.unwrap();
        assert!(fetched_deleted.deleted);
    }
}
