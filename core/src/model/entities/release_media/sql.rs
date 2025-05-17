use super::{ReleaseMedia, op::ReleaseMediaUpdateOp};

crate::dsot_sql_entity!(["release_media"] ReleaseMedia with ReleaseMediaUpdateOp {
    release_id,
    format,
    count
});

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();
    }
}
