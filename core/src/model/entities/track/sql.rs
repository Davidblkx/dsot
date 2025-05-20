use super::{Track, op::TrackUpdateOp};

crate::dsot_sql_entity!(["tracks"] Track with TrackUpdateOp {
    mbid,
    title,
    release_media_id,
    media_index,
    release_index,
    track_number,
    position,
    recording_id
});

#[cfg(test)]
mod tests {
    use crate::model::entities::recording::{Recording, sql::RecordingSql};
    use crate::model::entities::release_media::{ReleaseMedia, sql::ReleaseMediaSql};
    use crate::model::entities::album::{Album, sql::AlbumSql};
    use crate::model::entities::release::{Release, sql::ReleaseSql};

    use super::*;

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let album = Album::new("my dupper album", 2023);
        let release = Release::for_album(&album);
        let rm = ReleaseMedia::new(&release.id, &(1.into()));
        let rec = Recording::new("my recording".to_string());
        let track = Track::new("my track".to_string(), &rm.id, &rec.id);

        let (trx, _) = AlbumSql::insert(trx, &album).await.unwrap();
        let (trx, _) = ReleaseSql::insert(trx, &release).await.unwrap();
        let (trx, _) = ReleaseMediaSql::insert(trx, &rm).await.unwrap();
        let (trx, _) = RecordingSql::insert(trx, &rec).await.unwrap();
        let (trx, _) = TrackSql::insert(trx, &track).await.unwrap();

        let (_, rel) = TrackSql::fetch_by_id(trx, &track.id).await.unwrap();

        let res = rel.unwrap();
        assert_eq!(res.release_media_id, track.release_media_id);
        assert_eq!(res.media_index, track.media_index);
        assert_eq!(res.release_index, track.release_index);
        assert_eq!(res.track_number, track.track_number);
        assert_eq!(res.position, track.position);
        assert_eq!(res.title, track.title);
        assert_eq!(res.mbid, track.mbid);
        assert_eq!(res.recording_id, track.recording_id);
        assert_eq!(res.id, track.id);
    }
}
