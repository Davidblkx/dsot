use crate::model::artist::ArtistCredit;
use crate::model::Relationship;

entity!(Recording{
    inc: [artists, releases, release_groups="release-groups", isrcs, url_rels="url-rels", work_rels="work-rels"],
    search [recordings]: {
        alias: "Part of any alias attached to the recording (diacritics are ignored)",
        arid: "The MBID of any of the recording artists",
        artist: "Part of the combined credited artist name for the recording, including join phrases (e.g. 'Artist X feat.')",
        artistname: "Part of the name of any of the recording artists",
        comment: "Part of the recording's disambiguation comment",
        country: "The 2-letter code (ISO 3166-1 alpha-2) for the country any release of this recording was released in",
        creditname: "Part of the credited name of any of the recording artists on this particular recording",
        date: "The release date of any release including this recording (e.g. '1980-01-22')",
        dur: "The recording duration in milliseconds",
        firstreleasedate: "The release date of the earliest release including this recording (e.g. '1980-01-22')",
        format: "The format of any medium including this recording (insensitive to case, spaces, and separators)",
        isrc: "Any ISRC associated to the recording",
        number: "The free-text number of the track on any medium including this recording (e.g. 'A4')",
        position: "The position inside its release of any medium including this recording (starts at 1)",
        primarytype: "The primary type of any release group including this recording",
        qdur: "The recording duration, quantized (duration in milliseconds / 2000)",
        recording: "Part of the recording's name, or the name of a track connected to this recording (diacritics are ignored)",
        recordingaccent: "Part of the recordings's name, or the name of a track connected to this recording (with the specified diacritics)",
        reid: "The MBID of any release including this recording",
        release: "Part of the name of any release including this recording",
        rgid: "The MBID of any release group including this recording",
        rid: "The recording's MBID",
        secondarytype: "Any of the secondary types of any release group including this recording",
        status: "The status of any release including this recording",
        tag: "Part of a tag attached to the recording",
        tid: "The MBID of a track connected to this recording",
        tnum: "The position of the track on any medium including this recording (starts at 1, pre-gaps at 0)",
        tracks: "The number of tracks on any medium including this recording",
        tracksrelease: "The number of tracks on a release (as a whole) including this recording",
        video: "A boolean flag (true/false) indicating whether or not the recording is a video recording"
    },
    schema ["recording"]: {
        ["The title of the recording."]
        title: String,
        ["The artist(s) that the recording is primarily credited to."]
        artists "artist-credit": Option<Vec<ArtistCredit>>,
        ["The length of the recording in milliseconds."]
        length: Option<u64>,
        ["The ISRC (International Standard Recording Code) of the recording."]
        isrcs: Option<Vec<String>>,
        ["The disambiguation comment."]
        disambiguation: Option<String>,
        ["The date of the earliest release that includes the recording. e.g. '1980-01-22'"]
        first_release_date "first-release-date": Option<String>,
        ["Indicating whether or not the recording is a video recording"]
        video: Option<bool>,
        ["Included relationships to other entities."]
        relations: Option<Vec<Relationship>>
    }
});
