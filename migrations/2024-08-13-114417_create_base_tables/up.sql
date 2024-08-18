-- Main entities
CREATE TABLE IF NOT EXISTS files (
    id BLOB NOT NULL PRIMARY KEY,
    path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tag_types (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tags (
    id BLOB NOT NULL PRIMARY KEY,
    type_id BLOB NOT NULL REFERENCES tag_types(id) ON DELETE CASCADE,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS artists (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    aliases BLOB NOT NULL,
    mbid BLOB
);

CREATE TABLE IF NOT EXISTS albums (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    mbid BLOB,
    aliases BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS releases (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    album_id BLOB NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
    date BIGINT NOT NULL,
    year BIGINT NOT NULL,
    tracks BLOB NOT NULL,
    mbid BLOB
);

CREATE TABLE IF NOT EXISTS tracks (
    id BLOB NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    file_id BLOB REFERENCES files(id) ON DELETE SET NULL
);
-- End entities

-- Relations
CREATE TABLE IF NOT EXISTS file_tags (
    file_id BLOB NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    tag_id BLOB NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS artist_tags (
    artist_id BLOB NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    tag_id BLOB NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS album_tags (
    album_id BLOB NOT NULL REFERENCES albums(id) ON DELETE CASCADE,
    tag_id BLOB NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS release_tags (
    release_id BLOB NOT NULL REFERENCES releases(id) ON DELETE CASCADE,
    tag_id BLOB NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS track_tags (
    track_id BLOB NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    tag_id BLOB NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS artist_albums (
    artist_id BLOB NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    album_id BLOB NOT NULL REFERENCES albums(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS artist_tracks (
    artist_id BLOB NOT NULL REFERENCES artists(id) ON DELETE CASCADE,
    track_id BLOB NOT NULL REFERENCES tracks(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS release_tracks (
    release_id BLOB NOT NULL REFERENCES releases(id) ON DELETE CASCADE,
    track_id BLOB NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    media_index BIGINT NOT NULL,
    track_index BIGINT NOT NULL
);
-- End relations

-- Indexes
CREATE INDEX IF NOT EXISTS tags_type_id ON tags(type_id);
CREATE INDEX IF NOT EXISTS artists_mbid ON artists(mbid);
CREATE INDEX IF NOT EXISTS albums_mbid ON albums(mbid);
CREATE INDEX IF NOT EXISTS releases_mbid ON releases(mbid);
CREATE INDEX IF NOT EXISTS releases_album_id ON releases(album_id);
CREATE INDEX IF NOT EXISTS tracks_file_id ON tracks(file_id);

CREATE INDEX IF NOT EXISTS file_tags_file_id ON file_tags(file_id);
CREATE INDEX IF NOT EXISTS file_tags_tag_id ON file_tags(tag_id);

CREATE INDEX IF NOT EXISTS artist_tags_artist_id ON artist_tags(artist_id);
CREATE INDEX IF NOT EXISTS artist_tags_tag_id ON artist_tags(tag_id);

CREATE INDEX IF NOT EXISTS album_tags_album_id ON album_tags(album_id);
CREATE INDEX IF NOT EXISTS album_tags_tag_id ON album_tags(tag_id);

CREATE INDEX IF NOT EXISTS artist_albums_artist_id ON artist_albums(artist_id);
CREATE INDEX IF NOT EXISTS artist_albums_album_id ON artist_albums(album_id);

CREATE INDEX IF NOT EXISTS release_tracks_release_id ON release_tracks(release_id);
CREATE INDEX IF NOT EXISTS release_tracks_track_id ON release_tracks(track_id);

CREATE INDEX IF NOT EXISTS track_tags_track_id ON track_tags(track_id);
CREATE INDEX IF NOT EXISTS track_tags_tag_id ON track_tags(tag_id);

CREATE INDEX IF NOT EXISTS artist_tracks_artist_id ON artist_tracks(artist_id);
CREATE INDEX IF NOT EXISTS artist_tracks_track_id ON artist_tracks(track_id);
-- End indexes
