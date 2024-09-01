search_query!(Artist ["artist"] {
    alias: "Part of any alias attached to the artist (diacritics are ignored)",
    primary_alias: "Part of any primary alias attached to the artist (diacritics are ignored)",
    area: "Part of the name of the artist's main associated area",
    arid: "The artist's MBID",
    artist: "Part of the artist's name (diacritics are ignored)",
    artistaccent: "Part of the artist's name (with the specified diacritics)",
    begin: "The artist's begin date (e.g. \"1980-01-22\")",
    beginarea: "Part of the name of the artist's begin area",
    comment: "Part of the artist's disambiguation comment",
    country: "The 2-letter code (ISO 3166-1 alpha-2) for the artist's main associated country",
    end: "The artist's end date (e.g. \"1980-01-22\")",
    endarea: "Part of the name of the artist's end area",
    ended: "A boolean flag (true/false) indicating whether or not the artist has ended (is dissolved/deceased)",
    gender: "The artist's gender (“male”, “female”, “other” or “not applicable”)",
    ipi: "An IPI code associated with the artist",
    isni: "An ISNI code associated with the artist",
    sortname: "Part of the artist's sort name",
    tag: "Part of a tag attached to the artist"
});


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {

        let query = ArtistQueryBuilder::new()
            .begin_group()
            .artist("name")
            .or()
            .artist("name2")
            .end_group()
            .build();

        assert_eq!(query.value, "(artist:name OR artist:name2)");
    }
}
