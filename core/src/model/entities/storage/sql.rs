use super::{Storage, op::StorageUpdateOp};

crate::dsot_sql_entity!(["storages"] Storage with StorageUpdateOp {
    description,
    mount,
    root,
    serial_number,
    is_default
});

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let storage = Storage::new(
            "Test Storage".to_string(),
            "/mnt/test_storage".to_string(),
            "media/music".to_string(),
            "1234567890".to_string(),
            true,
        );

        let (trx, _) = StorageSql::insert(trx, &storage).await.unwrap();

        let (_, fetched_storage) = StorageSql::fetch_by_id(trx, &storage.id).await.unwrap();
        let res = fetched_storage.unwrap();

        assert_eq!(res.description, "Test Storage");
        assert_eq!(res.mount, "/mnt/test_storage");
        assert_eq!(res.root, "media/music");
        assert_eq!(res.serial_number, "1234567890");
        assert!(res.is_default);
    }
}
