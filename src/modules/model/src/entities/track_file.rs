use dsot_derive::SyncEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Default, SyncEntity)]
#[table(track_files)]
pub struct TrackFile {
    pub id: Uuid,
    pub recording_id: Uuid,
}

impl TrackFile {
    pub fn new(id: Uuid, recording_id: Uuid) -> Self {
        Self { id, recording_id }
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
    async fn test_track_file_crud(pool: SqlitePool) {
        let db = setup_database(pool);
        let id = Uuid::now_v7();
        let recording_id = Uuid::now_v7();
        let track_file = TrackFile::new(id, recording_id);

        // Test insert
        let sync_entity = track_file.to_sync();
        db.insert::<TrackFileSqlRepository>(&sync_entity)
            .await
            .unwrap();

        // Test get
        let fetched = db.get::<TrackFileSqlRepository>(id).await.unwrap();
        assert_eq!(fetched.recording_id, recording_id);

        // Test update
        let mut updated = fetched.clone();
        let new_recording_id = Uuid::now_v7();
        updated.recording_id = new_recording_id;

        let op = updated.op_update(&fetched).expect("change detected");
        db.apply_journal::<TrackFileSqlRepository>(JournalEntry::new("track_files", &op))
            .await
            .unwrap();

        let fetched_updated = db.get::<TrackFileSqlRepository>(id).await.unwrap();
        assert_eq!(fetched_updated.recording_id, new_recording_id);

        // Test delete
        db.delete::<TrackFileSqlRepository>(id).await.unwrap();
        let fetched_deleted = db.get::<TrackFileSqlRepository>(id).await.unwrap();
        assert!(fetched_deleted.deleted);
    }
}
