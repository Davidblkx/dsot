use super::{MusicFile, op::MusicFileUpdateOp};

crate::dsot_sql_entity!(["music_files"] MusicFile with MusicFileUpdateOp {
    path,
    storage_id,
    recording_id,
    size,
    format,
    need_better,
    chromaprint
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::entities::music_file::MusicFileFormat;
    use crate::model::entities::storage::{Storage, sql::StorageSql};

    #[sqlx::test(migrations = "../migrations")]
    async fn can_query(pool: sqlx::SqlitePool) {
        let trx = pool.begin().await.unwrap();

        let storage = Storage::new("local".to_string());
        let mut music = MusicFile::new(&storage.id, "album/song.mp3");
        music.size = 123456;
        music.set_format(MusicFileFormat::FLAC);
        music.chromaprint = Some("abcdef1234567890".to_string());


        let (trx, _) = StorageSql::insert(trx, &storage).await.unwrap();
        let (trx, _) = MusicFileSql::insert(trx, &music).await.unwrap();

        let (_, fetched_music) = MusicFileSql::fetch_by_id(trx, &music.id).await.unwrap();
        let res = fetched_music.unwrap();

        assert_eq!(res.path, music.path);
        assert_eq!(res.storage_id, music.storage_id);
        assert_eq!(res.size, music.size);
        assert_eq!(res.format, music.format);
        assert_eq!(res.chromaprint, music.chromaprint);
        assert_eq!(res.get_format(), MusicFileFormat::FLAC);
        assert_eq!(res.recording_id, music.recording_id);
    }
}
