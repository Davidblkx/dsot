macro_rules! mb_entity {
    ($name:ident, $(
        comment = $comment:expr,
        prop = $prop:ident,
        $(json = $json_prop:literal)?
        value = $prop_type:ty,
    ),*) => {
        #[derive(Clone, Debug, serde::Deserialize)]
        pub struct $name {
            #[doc = "The MusicBrainz Identifier (MBID) of the entity."]
            pub id: String,

            $(
                $(#[serde(alias = $json_prop)])?
                #[serde(default)]
                #[doc = $comment]
                pub $prop: $prop_type,
            )*

            #[doc = "The entity's search score, only available when searching."]
            pub score: Option<u32>,

            #[doc = "Tags for the entity."]
            pub tags: Option<Vec<crate::model::Tag>>,

            #[doc = "Annotation for the entity."]
            pub annotation: Option<String>,

            #[doc = "Genres for the entity."]
            pub genres: Option<Vec<crate::entities::Genre>>,
        }
    };
}
