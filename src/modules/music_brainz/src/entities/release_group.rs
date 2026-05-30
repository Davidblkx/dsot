use crate::model::{artist::ArtistCredit, release_group::{ReleaseGroupType, ReleaseGroupSubType}};

entity!(ReleaseGroup {
    inc: [artists, releases],
    search [release_groups, "release-groups"]: {
        alias: "Part of any alias attached to the release group (diacritics are ignored)",
        arid: "The MBID of any of the release group artists",
        artist: "Part of the combined credited artist name for the release group, including join phrases (e.g. 'Artist X feat.')",
        artistname: "Part of the name of any of the release group artists",
        comment: "Part of the release group's disambiguation comment",
        creditname: "Part of the credited name of any of the release group artists on this particular release group",
        firstreleasedate: "The release date of the earliest release in this release group (e.g. '1980-01-22')",
        primarytype: "The primary type of the release group",
        reid: "The MBID of any of the releases in the release group",
        release: "Part of the title of any of the releases in the release group",
        releasegroup: "Part of the release group's title (diacritics are ignored)",
        releasegroupaccent: "Part of the release group's title (with the specified diacritics)",
        releases: "The number of releases in the release group",
        rgid: "The release group's MBID",
        secondarytype: "Any of the secondary types of the release group",
        status: "The status of any of the releases in the release group",
        tag: "Part of a tag attached to the release group"
    },
    schema ["release-group"]: {
        ["The title of a release group is usually very similar, if not the same, as the titles of the releases contained within it."]
        title: String,
        ["The artist of a release group is usually very similar, if not the same, as the artist of the releases contained within it. Multiple artists can be linked using artist credits."]
        artists "artist-credit": Option<Vec<ArtistCredit>>,
        ["The main type of a release group, e.g. album, single"]
        primary_type "primary-type": Option<ReleaseGroupType>,
        ["A secondary type of a release group, e.g. compilation, soundtrack"]
        secondary_types "secondary-types": Option<Vec<ReleaseGroupSubType>>,
        ["The release group's disambiguation comment"]
        disambiguation: Option<String>,
        ["The release group's primary type id"]
        primary_type_id "primary-type-id": Option<String>,
        ["The release group's secondary type ids"]
        secondary_type_ids "secondary-type-ids": Option<Vec<String>>,
        ["The first release date of the release group"]
        date "first-release-date": Option<String>
    }
});

impl ReleaseGroup {
    pub fn main_artist(&self) -> Option<&ArtistCredit> {
        self.artists.as_ref().and_then(|artists| artists.first())
    }
}
