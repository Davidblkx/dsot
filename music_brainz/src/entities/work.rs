entity!(Work{
    inc: [recording_rels="recording-rels"],
    search [works]: {
        alias: "Part of any alias attached to the work (diacritics are ignored)",
        arid: "	the MBID of an artist related to the work (e.g. a composer or lyricist) ",
        artist: "(part of) the name of an artist related to the work (e.g. a composer or lyricist)",
        comment: "Part of the work's disambiguation comment",
        iswc: "any ISWC (International Standard Musical Work Code) associated to the work",
        lang: "the ISO 639-3 code for the work language",
        recording: "Part of the title of a recording related to the work (diacritics are ignored)",
        recording_count: "the number of recordings related to the work",
        rid: "The MBID of a recording related to the work",
        tag: "Part of a tag attached to the work",
        r#type: "the type of the work (e.g. 'song', 'musical', 'libretto')",
        wid: "The MBID of the work",
        work: "Part of the title of the work (diacritics are ignored)",
        workaccent: "Part of the title of the work (with the specified diacritics)"
    },
    schema ["work"]: {
        ["The title of the work."]
        title: String,
        ["The ISWC (International Standard Musical Work Code) of the work."]
        iswcs: Option<Vec<String>>,
        ["The disambiguation comment."]
        disambiguation: Option<String>,
        ["The language of the work."]
        language: Option<String>,
        ["The type of the work."]
        r#type: Option<String>,
        ["The list of all languages the work is available in."]
        languages: Option<Vec<String>>
    }
});
