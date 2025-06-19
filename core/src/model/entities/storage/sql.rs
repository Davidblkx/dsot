use super::{Storage, op::StorageUpdateOp};

crate::dsot_sql_entity!(["storages"] Storage with StorageUpdateOp {
    description: Option<String>,
    kind: String,
    path: Option<String>,
    info: Option<String>
});

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let storage = Storage {
            description: Some("Test Storage".to_string()),
            id: uuid::Uuid::now_v7(),
            kind: "local".to_string(),
            info: Some("Extra info about storage".to_string()),
            path: Some("media/music".to_string()),
        };

        let (trx, _) = StorageSql::insert(trx, &storage).await.unwrap();

        let (_, fetched_storage) = StorageSql::fetch_by_id(trx, &storage.id).await.unwrap();
        let res = fetched_storage.unwrap();

        assert_eq!(res.id, storage.id);
        assert_eq!(res.description, storage.description);
        assert_eq!(res.kind, storage.kind);
        assert_eq!(res.info, storage.info);
        assert_eq!(res.path, storage.path);
    }
}
