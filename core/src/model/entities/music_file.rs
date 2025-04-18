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

    #[test]
    fn generates_expected_insert_sql() {
        let result = MusicFile::get_sql_insert_statement();
        let expected = "INSERT INTO music_files (id, path) VALUES (?, ?)";
        assert_eq!(result, expected);
    }

    #[test]
    fn generates_expected_fetch_sql() {
        let result = MusicFile::get_sql_fetch_by_id_statement();
        let expected = "SELECT id, path FROM music_files WHERE id = ?";
        assert_eq!(result, expected);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn can_do_insert_query(pool: SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let music_file = MusicFile {
            id: Uuid::now_v7(),
            path: String::from("test_path"),
        };

        let trx = MusicFile::execute_sql_insert(trx, &music_file).await.unwrap();

        let result = MusicFile::execute_sql_fetch_by_id(trx, &music_file.id).await.unwrap();
        assert!(result.is_some());
        let fetched_music_file = result.unwrap();
        assert_eq!(fetched_music_file.id, music_file.id);
        assert_eq!(fetched_music_file.path, music_file.path);
    }
}
