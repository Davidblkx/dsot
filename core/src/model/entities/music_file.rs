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

    #[sqlx::test(migrations = "../migrations")]
    async fn can_do_sql_crud_operations(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let music_file = MusicFile {
            id: Uuid::now_v7(),
            path: String::from("test_path"),
        };

        // Insert
        let (trx, ()) = MusicFileSql::insert(trx, &music_file).await.unwrap();

        // Fetch by ID
        let (trx, result) = MusicFileSql::fetch_by_id(trx, &music_file.id).await.unwrap();
        let fetched_music_file = result.unwrap();
        assert_eq!(fetched_music_file.id, music_file.id);
        assert_eq!(fetched_music_file.path, music_file.path);

        // Update Path
        let (trx, ()) = MusicFileSql::update(
            trx,
            &music_file.id,
            &MusicFileUpdateOp::SetPath(String::from("new_path")),
        ).await.unwrap();

        // Fetch by ID again to check the updates
        let (trx, result) = MusicFileSql::fetch_by_id(trx, &music_file.id).await.unwrap();
        let fetched_music_file = result.unwrap();
        assert_eq!(fetched_music_file.path, "new_path");

        // Delete
        let (trx, ()) = MusicFileSql::delete(trx, &music_file.id).await.unwrap();

        // Check if the record is deleted
        let (_, result) = MusicFileSql::fetch_by_id(trx, &music_file.id).await.unwrap();
        assert!(result.is_none());
    }
}
