DROP INDEX IF EXISTS tags_type_id;
DROP INDEX IF EXISTS artists_mbid;
DROP INDEX IF EXISTS albums_mbid;
DROP INDEX IF EXISTS releases_mbid;
DROP INDEX IF EXISTS releases_album_id;
DROP INDEX IF EXISTS tracks_file_id;
DROP INDEX IF EXISTS tracks_mbid;
DROP INDEX IF EXISTS file_tags_file_id;
DROP INDEX IF EXISTS file_tags_tag_id;
DROP INDEX IF EXISTS artist_tags_artist_id;
DROP INDEX IF EXISTS artist_tags_tag_id;
DROP INDEX IF EXISTS album_tags_album_id;
DROP INDEX IF EXISTS album_tags_tag_id;
DROP INDEX IF EXISTS artist_albums_artist_id;
DROP INDEX IF EXISTS artist_albums_album_id;
DROP INDEX IF EXISTS release_tracks_release_id;
DROP INDEX IF EXISTS release_tracks_track_id;
-- End indexes

DROP TABLE IF EXISTS file_tags;
DROP TABLE IF EXISTS artist_tags;
DROP TABLE IF EXISTS album_tags;
DROP TABLE IF EXISTS release_tags;
DROP TABLE IF EXISTS track_tags;
DROP TABLE IF EXISTS artist_albums;
DROP TABLE IF EXISTS release_tracks;

DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS releases;
DROP TABLE IF EXISTS tracks;
DROP TABLE IF EXISTS albums;
DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS artists;
DROP TABLE IF EXISTS tag_types;
