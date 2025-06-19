use super::{Work, op::WorkUpdateOp};

crate::dsot_sql_entity!(["works"] Work with WorkUpdateOp {
    mbid: Option<uuid::Uuid>,
    title: String,
    kind: Option<String>,
    language: Option<String>,
    disambiguation: Option<String>
});

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let mut work = Work::new("Test Work".to_string());
        work.kind = Some("Composition".to_string());
        work.language = Some("English".to_string());

        let (trx, _) = WorkSql::insert(trx, &work).await.unwrap();
        let (_, fetched_work) = WorkSql::fetch_by_id(trx, &work.id).await.unwrap();
        let res = fetched_work.unwrap();

        assert_eq!(res.id, work.id);
        assert_eq!(res.title, work.title);
        assert_eq!(res.mbid, work.mbid);
        assert_eq!(res.kind, work.kind);
        assert_eq!(res.language, work.language);
        assert_eq!(res.disambiguation, work.disambiguation);
    }
}
