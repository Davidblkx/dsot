use super::{Inbox, op::InboxUpdateOp};

crate::dsot_sql_entity!(["inbox"] Inbox with InboxUpdateOp {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    file: Option<String>,
    extra_info: Option<String>
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

    #[sqlx::test(migrations = "../migrations")]
    async fn can_search(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let inbox1 = Inbox::new()
            .set_title("Test Inbox")
            .set_artist("Test Artist")
            .set_album("Test Album")
            .set_file("test_file.mp3")
            .set_extra_info("Some extra info");

        let inbox2 = Inbox::new()
            .set_title("Another Inbox")
            .set_artist("Another Artist")
            .set_album("Another Album")
            .set_file("another_file.mp3")
            .set_extra_info("Some other extra info");

        let (trx, _) = InboxSql::insert(trx, &inbox1).await.unwrap();
        let (trx, _) = InboxSql::insert(trx, &inbox2).await.unwrap();
        let (_, results) = InboxSql::search(trx, "Another*", 10, 0).await.unwrap();

        assert_eq!(results.len(), 1);
        let res = &results[0];
        assert_eq!(res.id, inbox2.id);
        assert_eq!(res.title, inbox2.title);
        assert_eq!(res.artist, inbox2.artist);
        assert_eq!(res.album, inbox2.album);
        assert_eq!(res.file, inbox2.file);
        assert_eq!(res.extra_info, inbox2.extra_info);
    }
}
