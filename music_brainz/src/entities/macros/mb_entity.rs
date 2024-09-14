macro_rules! mb_entity {
    ($name:ident {$(
        [$comment:tt]
        $prop:ident $($json_prop:literal)?: $prop_type:ty
    ),*}) => {
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
