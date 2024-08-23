-- Your SQL goes here
CREATE VIEW IF NOT EXISTS artist_albums_view AS
SELECT
    artists.id AS artist_id,
    albums.id AS album_id,
    artists.name AS artist_name,
    albums.name AS album_name,
    artists.mbid AS artist_mbid,
    albums.mbid AS album_mbid
FROM artists
LEFT JOIN artist_albums ON artists.id = artist_albums.artist_id
LEFT JOIN albums ON artist_albums.album_id = albums.id;
