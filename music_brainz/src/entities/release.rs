use crate::model::artist::ArtistCredit;

entity!(Release{
    inc: [artists, collections, labels, recordings, release_groups="release-groups"],
    search [releases]: {
        arid: "The MBID of any of the release artists",
        artist: "Part of the combined credited artist name for the release, including join phrases (e.g. 'Artist X feat.')",
        artistname: "Part of the name of any of the release artists",
        asin: "An Amazon ASIN for the release",
        barcode: "The barcode for the release",
        catno: "Any catalog number for this release (insensitive to case, spaces, and separators)",
        comment: "Part of the release's disambiguation comment",
        country: "The 2-letter code (ISO 3166-1 alpha-2) for any country the release was released in",
        creditname: "Part of the credited name of any of the release artists on this particular release",
        date: "A release date for the release (e.g. '1980-01-22')",
        discids: "The total number of disc IDs attached to all mediums on the release",
        discidsmedium: "The number of disc IDs attached to any one medium on the release",
        format: "The format of any medium in the release (insensitive to case, spaces, and separators)",
        laid: "The MBID of any of the release labels",
        label: "Part of the name of any of the release labels",
        lang: "The ISO 639-3 code for the release language",
        mediums: "The number of mediums on the release",
        packaging: "The format of the release (insensitive to case, spaces, and separators)",
        primarytype: "The primary type of the release group for this release",
        quality: "The listed quality of the data for the release (2 for 'high', 1 for 'normal'; cannot search for 'low' at the moment; see the related bug report)",
        reid: "The release's MBID",
        release: "Part of the release's title (diacritics are ignored)",
        releaseaccent: "Part of the release's title (with the specified diacritics)",
        rgid: "The MBID of the release group for this release",
        script: "The ISO 15924 code for the release script",
        secondarytype: "Any of the secondary types of the release group for this release",
        status: "The status of the release",
        tag: "Part of a tag attached to the release",
        tracks: "The total number of tracks on the release",
        tracksmedium: "The number of tracks on any one medium on the release"
    },
    schema ["release"]: {
        ["The title of the release."]
        title: String,
        ["The artist(s) that the release is primarily credited to."]
        artists "artist-credit": Option<Vec<ArtistCredit>>
    }
});
