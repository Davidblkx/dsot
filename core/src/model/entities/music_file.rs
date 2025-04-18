use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct MusicFileV0 {
    pub id: Uuid,
    pub path: String,
}

crate::dsot_storage_declare_model!(MusicFile {
    0: MusicFileV0
});

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MusicFileUpdateOpV0 {
    SetPath(String),
}

crate::dsot_storage_declare_model!(MusicFileUpdateOp {
    0: MusicFileUpdateOpV0
});

crate::dsot_sql_entity!(["music_files"] MusicFile with MusicFileUpdateOp {
    path
});

impl Default for MusicFile {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            path: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use crate::storage::sql::SqlEntity;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_do_sql_crud_operations(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let music_file = MusicFile {
            id: Uuid::now_v7(),
            path: String::from("test_path"),
        };

        // Insert
        let trx = MusicFile::execute_sql_insert(trx, &music_file).await.unwrap();

        // Fetch by ID
        let result = MusicFile::execute_sql_fetch_by_id(trx, &music_file.id).await.unwrap();
        let trx = result.0;
        let fetched_music_file = result.1.unwrap();
        assert_eq!(fetched_music_file.id, music_file.id);
        assert_eq!(fetched_music_file.path, music_file.path);

        // Update Path
        let trx = MusicFile::execute_sql_update(
            trx,
            &music_file.id,
            &MusicFileUpdateOp::SetPath(String::from("new_path")),
        ).await.unwrap();

        // Fetch by ID again to check the updates
        let result = MusicFile::execute_sql_fetch_by_id(trx, &music_file.id).await.unwrap();
        let trx = result.0;
        let fetched_music_file = result.1.unwrap();
        assert_eq!(fetched_music_file.path, "new_path");

        // Delete
        let trx = MusicFile::execute_sql_delete(trx, &music_file.id).await.unwrap();

        let result = MusicFile::execute_sql_fetch_by_id(trx, &music_file.id).await.unwrap();
        assert!(result.1.is_none());
    }
}
