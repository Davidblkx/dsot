use super::{Inbox, op::InboxUpdateOp};

crate::dsot_sql_entity!(["inbox"] Inbox with InboxUpdateOp {
    title,
    artist,
    album,
    file,
    extra_info
});

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let mut inbox = Inbox::new();
        inbox.title = Some("Test Inbox".to_string());
        inbox.artist = Some("Test Artist".to_string());
        inbox.album = Some("Test Album".to_string());
        inbox.file = Some("test_file.mp3".to_string());
        inbox.extra_info = Some("Some extra info".to_string());

        let (trx, _) = InboxSql::insert(trx, &inbox).await.unwrap();
        let (_, res) = InboxSql::fetch_by_id(trx, &inbox.id).await.unwrap();
        let res = res.unwrap();

        assert_eq!(res.id, inbox.id);
        assert_eq!(res.title, inbox.title);
        assert_eq!(res.artist, inbox.artist);
        assert_eq!(res.album, inbox.album);
        assert_eq!(res.file, inbox.file);
        assert_eq!(res.extra_info, inbox.extra_info);
    }
}
