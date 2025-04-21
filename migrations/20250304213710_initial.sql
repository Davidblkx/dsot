-- Add migration script here
CREATE TABLE artists (
    id BLOB PRIMARY KEY NOT NULL,
    mbid BLOB,
    name TEXT NOT NULL,
    sort_name TEXT,
    artist_type_id INTEGER NOT NULL
);

CREATE INDEX artists_mbid ON artists (mbid);

CREATE TABLE artist_aliases (
    id BLOB PRIMARY KEY NOT NULL,
    artist_id BLOB NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

CREATE INDEX artist_aliases_artist_id ON artist_aliases (artist_id);

CREATE TABLE albums (
    id BLOB PRIMARY KEY NOT NULL,
    mbid BLOB,
    title TEXT NOT NULL,
    year INTEGER NOT NULL
);

CREATE INDEX albums_mbid ON albums (mbid);

CREATE TABLE album_artists (
    id BLOB PRIMARY KEY NOT NULL,
    album_id BLOB NOT NULL,
    artist_id BLOB NOT NULL,
    is_main INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

CREATE INDEX album_artists_album_id ON album_artists (album_id);
CREATE INDEX album_artists_artist_id ON album_artists (artist_id);

CREATE TABLE music_files (
    id BLOB PRIMARY KEY NOT NULL,
    path TEXT NOT NULL
);
