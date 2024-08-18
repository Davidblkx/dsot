// @generated automatically by Diesel CLI.

diesel::table! {
    album_tags (rowid) {
        rowid -> Integer,
        album_id -> Binary,
        tag_id -> Binary,
    }
}

diesel::table! {
    albums (id) {
        id -> Binary,
        name -> Text,
        mbid -> Nullable<Binary>,
        aliases -> Binary,
    }
}

diesel::table! {
    artist_albums (rowid) {
        rowid -> Integer,
        artist_id -> Binary,
        album_id -> Binary,
    }
}

diesel::table! {
    artist_tags (rowid) {
        rowid -> Integer,
        artist_id -> Binary,
        tag_id -> Binary,
    }
}

diesel::table! {
    artist_tracks (rowid) {
        rowid -> Integer,
        artist_id -> Binary,
        track_id -> Binary,
    }
}

diesel::table! {
    artists (id) {
        id -> Binary,
        name -> Text,
        aliases -> Binary,
        mbid -> Nullable<Binary>,
    }
}

diesel::table! {
    file_tags (rowid) {
        rowid -> Integer,
        file_id -> Binary,
        tag_id -> Binary,
    }
}

diesel::table! {
    files (id) {
        id -> Binary,
        path -> Text,
    }
}

diesel::table! {
    release_tags (rowid) {
        rowid -> Integer,
        release_id -> Binary,
        tag_id -> Binary,
    }
}

diesel::table! {
    release_tracks (rowid) {
        rowid -> Integer,
        release_id -> Binary,
        track_id -> Binary,
        media_index -> BigInt,
        track_index -> BigInt,
    }
}

diesel::table! {
    releases (id) {
        id -> Binary,
        name -> Text,
        album_id -> Binary,
        date -> BigInt,
        year -> BigInt,
        tracks -> Binary,
        mbid -> Nullable<Binary>,
    }
}

diesel::table! {
    tag_types (id) {
        id -> Binary,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Binary,
        type_id -> Binary,
        name -> Text,
    }
}

diesel::table! {
    track_tags (rowid) {
        rowid -> Integer,
        track_id -> Binary,
        tag_id -> Binary,
    }
}

diesel::table! {
    tracks (id) {
        id -> Binary,
        name -> Text,
        file_id -> Nullable<Binary>,
    }
}

diesel::joinable!(album_tags -> albums (album_id));
diesel::joinable!(album_tags -> tags (tag_id));
diesel::joinable!(artist_albums -> albums (album_id));
diesel::joinable!(artist_albums -> artists (artist_id));
diesel::joinable!(artist_tags -> artists (artist_id));
diesel::joinable!(artist_tags -> tags (tag_id));
diesel::joinable!(artist_tracks -> artists (artist_id));
diesel::joinable!(artist_tracks -> tracks (track_id));
diesel::joinable!(file_tags -> files (file_id));
diesel::joinable!(file_tags -> tags (tag_id));
diesel::joinable!(release_tags -> releases (release_id));
diesel::joinable!(release_tags -> tags (tag_id));
diesel::joinable!(release_tracks -> releases (release_id));
diesel::joinable!(release_tracks -> tracks (track_id));
diesel::joinable!(releases -> albums (album_id));
diesel::joinable!(tags -> tag_types (type_id));
diesel::joinable!(track_tags -> tags (tag_id));
diesel::joinable!(track_tags -> tracks (track_id));
diesel::joinable!(tracks -> files (file_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_tags,
    albums,
    artist_albums,
    artist_tags,
    artist_tracks,
    artists,
    file_tags,
    files,
    release_tags,
    release_tracks,
    releases,
    tag_types,
    tags,
    track_tags,
    tracks,
);
