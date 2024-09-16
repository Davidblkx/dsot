macro_rules! entity {
    (
        $name:ident {
            inc: [$($inc:ident$(=$inc_value:expr)?),*],
            search [$search_result:ident]: {$($prop_search:ident: $comment_search:expr),*},
            schema [$schema_name:ident]: {$(
                [$comment:expr]
                $prop:ident $($json_prop:literal)?: $prop_type:ty
            ),*}
        }
    ) => {
        mb_entity!{$name, $(
            comment = $comment,
            prop = $prop,
            $(json = $json_prop)?
            value = $prop_type,
        ),*}

        mb_search!{$name {
            name = $schema_name,
            search_name = $search_result,
            props = {$($prop_search: $comment_search),*}
        }}

        mb_lookup!{$name {
            target = $schema_name,
            inc = $($inc$(=$inc_value)?),*
        }}
    };
}
