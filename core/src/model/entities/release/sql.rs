use super::{Release, op::ReleaseUpdateOp};
use crate::model::entities::album::{Album, sql::AlbumSql};

crate::dsot_sql_entity!(["releases"] Release with ReleaseUpdateOp {
    mbid: Option<uuid::Uuid>,
    title: String,
    year: Option<i32>,
    status: Option<u32>,
    country: Option<String>,
    duration: Option<i64>,
    format: Option<String>,
    album_id: uuid::Uuid
});

impl Release {
    /// Fetches the album associated with this release.
    pub async fn get_album(&self, trx: SqlTransaction) -> SqlResult<Album> {
        let (trx, album) = AlbumSql::fetch_by_id(trx, &self.album_id).await?;
        if let Some(album) = album {
            Ok((trx, album))
        } else {
            Err(crate::error::DsotError::SqlMissingRelation(format!(
                "Album with ID {} not found",
                self.album_id
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query_album(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let album = Album::new("my dupper album", 2023);
        let release = Release::for_album(&album);

        let (trx, _) = AlbumSql::insert(trx, &album).await.unwrap();
        let (trx, _) = ReleaseSql::insert(trx, &release).await.unwrap();

        let (_, rel) = release.get_album(trx).await.unwrap();

        assert_eq!(rel.title, album.title);
    }
}
