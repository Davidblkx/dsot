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
                #[doc = $comment]
                pub $prop: $prop_type,
            )*
        }
    };
}
