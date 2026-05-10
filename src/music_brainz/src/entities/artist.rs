use crate::model::{LifeSpan, Alias, artist::ArtistType};
use super::Area;

entity!(Artist{
    inc: [aliases, recordings, releases, release_group="release-group", works, various_artists="various-artists"],
    search [artists]: {
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
        tag: "Part of a tag attached to the artist",
        r#type: "The artist's type (e.g. 'Person', 'Group', ...)"
    },
    schema ["artist"]: {
        ["The official name of an artist, be it a person or a band."]
        name: String,
        ["The sort name is a variant of the artist's name which would be used when sorting artists by name."]
        sort_name "sort-name": Option<String>,
        ["The type is used to state whether an artist is a person, a group, or something else."]
        r#type: Option<ArtistType>,
        ["The gender is used to explicitly state whether a person or character identifies as male, female or neither. Groups do not have genders."]
        gender: Option<String>,
        ["The gender identifier"]
        gender_id: Option<String>,
        ["The artist area, as the name suggests, indicates the area with which an artist is primarily identified with. It is often, but not always, its birth/formation country. "]
        area: Option<Area>,
        ["The begin and end dates indicate when an artist started and ended its existence."]
        life_span "life-span": Option<LifeSpan>,
        ["An IPI (interested party information) code is an identifying number assigned by the CISAC database for musical rights management."]
        ipis: Option<Vec<String>>,
        ["The International Standard Name Identifier for the artist"]
        isnis: Option<Vec<String>>,
        ["The artist's aliases."]
        aliases: Option<Vec<Alias>>,
        ["The artist's disambiguation comment."]
        disambiguation: Option<String>,
        ["Where the artist was born or formed."]
        begin_area "begin-area": Option<Area>,
        ["Where the artist died or was dissolved."]
        end_area "end-area": Option<Area>
    }
});
