use crate::model::{LifeSpan, area::{AreaType, AreaAlias}};

entity!(Area {
    inc: [aliases, area_rels="area-rels"],
    search [areas]: {
        alias: "Part of any alias attached to the artist (diacritics are ignored)",
        area: "Part of the area's name (diacritics are ignored)",
        areaaccent: "Part of the area's name (with the specified diacritics)",
        begin: "The area's begin date (e.g. \"1980-01-22\")",
        comment: "Part of the area's disambiguation comment",
        end: "The area's end date (e.g. \"1980-01-22\")",
        ended: "A boolean flag (true/false) indicating whether or not the area has ended (is no longer current)",
        iso: "An ISO 3166-1, 3166-2 or 3166-3 code attached to the area",
        iso1: "An ISO 3166-1 code attached to the area",
        iso2: "An ISO 3166-2 code attached to the area",
        iso3: "An ISO 3166-3 code attached to the area",
        sortname: "Equivalent to name (areas no longer have separate sort names)",
        tag: "Part of a tag attached to the area",
        r#type: "The area's type"
    },
    schema [area]: {
        ["An area is a geographic region or settlement. Areas are distinct from places, which are more specific and may contain areas."]
        name: String,
        ["The type of area."]
        r#type: Option<AreaType>,
        ["The sort name is a variant of the area's name which would be used when sorting areas by name."]
        sort_name "sort-name": Option<String>,
        ["The area's life span."]
        life_span "life-span": Option<LifeSpan>,
        ["iso-3166-1-codes for the area."]
        iso_3166_1_codes "iso-3166-1-codes": Option<Vec<String>>,
        ["iso-3166-2-codes for the area."]
        iso_3166_2_codes "iso-3166-2-codes": Option<Vec<String>>,
        ["iso-3166-3-codes for the area."]
        iso_3166_3_codes "iso-3166-3-codes": Option<Vec<String>>,
        ["The area's aliases."]
        aliases: Option<Vec<AreaAlias>>,
        ["The area's disambiguation comment."]
        disambiguation: Option<String>
    }
});
