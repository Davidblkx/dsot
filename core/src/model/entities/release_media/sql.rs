use super::{ReleaseMedia, op::ReleaseMediaUpdateOp};

crate::dsot_sql_entity!(["release_media"] ReleaseMedia with ReleaseMediaUpdateOp {
    release_id: uuid::Uuid,
    position: u32,
    format: u32,
    count: u32,
    mbid: Option<uuid::Uuid>
});

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::entities::album::{Album, sql::AlbumSql};
    use crate::model::entities::release::{Release, sql::ReleaseSql};

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let album = Album::new("my dupper album", 2023);
        let release = Release::for_album(&album);
        let rm = ReleaseMedia::new(&release.id, &(1.into()));

        let (trx, _) = AlbumSql::insert(trx, &album).await.unwrap();
        let (trx, _) = ReleaseSql::insert(trx, &release).await.unwrap();
        let (trx, _) = ReleaseMediaSql::insert(trx, &rm).await.unwrap();

        let (_, rel) = ReleaseMediaSql::fetch_by_id(trx, &rm.id).await.unwrap();
        let res = rel.unwrap();

        assert_eq!(res.release_id, rm.release_id);
        assert_eq!(res.format, rm.format);
        assert_eq!(res.count, rm.count);
    }
}
