use super::{Recording, op::RecordingUpdateOp};

crate::dsot_sql_entity!(["recordings"] Recording with RecordingUpdateOp {
    mbid,
    title,
    length,
    isrc,
    work_id,
    year,
    disambiguation
});

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let rec = Recording {
            id: uuid::Uuid::now_v7(),
            mbid: None,
            title: "my dupper recording".to_string(),
            length: Some(10),
            isrc: Some("my_isrc".to_string()),
            work_id: None,
            year: Some(2023),
            disambiguation: Some("my disambiguation".to_string()),
        };

        let (trx, _) = RecordingSql::insert(trx, &rec).await.unwrap();
        let (_, result) = RecordingSql::fetch_by_id(trx, &rec.id).await.unwrap();

        let res = result.unwrap();

        assert_eq!(res.id, rec.id);
        assert_eq!(res.mbid, rec.mbid);
        assert_eq!(res.title, rec.title);
        assert_eq!(res.length, rec.length);
        assert_eq!(res.isrc, rec.isrc);
        assert_eq!(res.work_id, rec.work_id);
        assert_eq!(res.year, rec.year);
        assert_eq!(res.disambiguation, rec.disambiguation);
    }
}
