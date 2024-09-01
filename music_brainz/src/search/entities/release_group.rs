search_query!(ReleaseGroup ["release-group"] {
    alias: "Any alias attached to the release group (diacritics are ignored)",
    arid: "The MBID of any of the release group artists",
    artist: "The combined credited artist name for the release group, including join phrases (e.g. 'Artist X feat.')",
    artistname: "The name of any of the release group artists",
    comment: "The release group's disambiguation comment",
    creditname: "The credited name of any of the release group artists on this particular release group",
    firstreleasedate: "The release date of the earliest release in this release group (e.g. '1980-01-22')",
    primarytype: "The primary type of the release group",
    reid: "The MBID of any of the releases in the release group",
    release: "The title of any of the releases in the release group",
    releasegroup: "The release group's title (diacritics are ignored)",
    releasegroupaccent: "The release group's title (with the specified diacritics)",
    releases: "The number of releases in the release group",
    rgid: "The release group's MBID",
    secondarytype: "Any of the secondary types of the release group",
    status: "The status of any of the releases in the release group",
    tag: "A tag attached to the release group"
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let query = ReleaseGroupQueryBuilder::new()
            .alias("alias_value")
            .and()
            .arid("arid_value")
            .and()
            .artistname("artist")
            .build();
        assert_eq!(query.value, "alias:alias_value AND arid:arid_value AND artistname:artist");
    }
}
