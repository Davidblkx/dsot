use super::{Inbox, op::InboxUpdateOp};

crate::dsot_sql_entity!(["inbox"] Inbox with InboxUpdateOp {
    mbid
});

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();
    }
}