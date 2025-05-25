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
    year INTEGER
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

CREATE TABLE releases (
    id BLOB PRIMARY KEY NOT NULL,
    mbid BLOB,
    title TEXT NOT NULL,
    year INTEGER,
    status INTEGER,
    country TEXT,
    duration BIGINT,
    format TEXT,
    album_id BLOB NOT NULL,
    FOREIGN KEY (album_id) REFERENCES albums (id) ON DELETE CASCADE
);

CREATE INDEX releases_mbid ON releases (mbid);
CREATE INDEX releases_album_id ON releases (album_id);

CREATE TABLE release_artists (
    id BLOB PRIMARY KEY NOT NULL,
    release_id BLOB NOT NULL,
    artist_id BLOB NOT NULL,
    FOREIGN KEY (release_id) REFERENCES releases (id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

CREATE INDEX release_artists_release_id ON release_artists (release_id);
CREATE INDEX release_artists_artist_id ON release_artists (artist_id);

CREATE TABLE release_media (
    id BLOB PRIMARY KEY NOT NULL,
    release_id BLOB NOT NULL,
    position BIGINT NOT NULL DEFAULT 0,
    format BIGINT NOT NULL DEFAULT 0,
    count BIGINT NOT NULL DEFAULT 0, -- track count
    mbid BLOB,
    FOREIGN KEY (release_id) REFERENCES releases (id) ON DELETE CASCADE
);

CREATE INDEX release_media_release_id ON release_media (release_id);

CREATE TABLE tracks (
    id BLOB PRIMARY KEY NOT NULL,
    release_media_id BLOB NOT NULL,
    media_index INTEGER NOT NULL, -- Index of the media (CD1, CD2, etc.)
    release_index INTEGER NOT NULL, -- Overall index in the release
    track_number INTEGER NOT NULL, -- Track number in the media
    position TEXT, -- Position in the media (e.g., "SideA-1")
    title TEXT NOT NULL,
    mbid BLOB,
    recording_id BLOB,
    FOREIGN KEY (release_media_id) REFERENCES release_media (id) ON DELETE CASCADE,
    FOREIGN KEY (recording_id) REFERENCES recordings (id) ON DELETE CASCADE
);

CREATE INDEX tracks_release_media_id ON tracks (release_media_id);
CREATE INDEX tracks_release_index ON tracks (release_index);

CREATE TABLE works (
    id BLOB PRIMARY KEY NOT NULL,
    mbid BLOB,
    title TEXT NOT NULL,
    kind TEXT, -- 1: Song, 2: Composition, etc.
    language TEXT,
    disambiguation TEXT
);

CREATE INDEX works_mbid ON works (mbid);

CREATE TABLE recordings (
    id BLOB PRIMARY KEY NOT NULL,
    mbid BLOB,
    title TEXT NOT NULL,
    length BIGINT,
    isrc TEXT,
    work_id BLOB,
    year INTEGER,
    disambiguation TEXT,
    FOREIGN KEY (work_id) REFERENCES works (id) ON DELETE CASCADE
);

CREATE INDEX recordings_mbid ON recordings (mbid);

CREATE TABLE recordings_artists (
    id BLOB PRIMARY KEY NOT NULL,
    recording_id BLOB NOT NULL,
    artist_id BLOB NOT NULL,
    FOREIGN KEY (recording_id) REFERENCES recordings (id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
);

CREATE INDEX recordings_artists_recording_id ON recordings_artists (recording_id);
CREATE INDEX recordings_artists_artist_id ON recordings_artists (artist_id);

CREATE TABLE storages (
    id BLOB PRIMARY KEY NOT NULL,
    description TEXT NOT NULL, -- Description of the storage device, e.g., "Music Storage 1"
    mount TEXT NOT NULL, -- Mount point of the storage device, linux "/mnt/storage1" or windows "D:\"
    root TEXT NOT NULL, -- Root directory of the storage device, e.g., "/mnt/storage1/music" or "D:\Music"
    serial_number TEXT NOT NULL, -- Serial number of the storage device
    is_default INTEGER NOT NULL DEFAULT 0 -- Whether this is the default storage
);

CREATE INDEX storages_mount ON storages (mount);
CREATE INDEX storages_serial_number ON storages (serial_number);
CREATE INDEX storages_is_default ON storages (is_default);

CREATE TABLE music_files (
    id BLOB PRIMARY KEY NOT NULL,
    path TEXT NOT NULL, -- path relative to the storage root, e.g., "Artist/Album/Track.mp3"
    storage_id BLOB NOT NULL,
    recording_id BLOB,
    size BIGINT NOT NULL,
    format INTEGER NOT NULL,
    is_lossless INTEGER NOT NULL DEFAULT 0,
    need_better INTEGER NOT NULL DEFAULT 0,
    chromaprint TEXT,
    FOREIGN KEY (storage_id) REFERENCES storages (id) ON DELETE CASCADE,
    FOREIGN KEY (recording_id) REFERENCES recordings (id) ON DELETE SET NULL
);

CREATE INDEX music_files_path ON music_files (path);
CREATE INDEX music_files_storage_id ON music_files (storage_id);
CREATE INDEX music_files_recording_id ON music_files (recording_id);
