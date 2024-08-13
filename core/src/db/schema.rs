// @generated automatically by Diesel CLI.

diesel::table! {
    file_tags (rowid) {
        rowid -> Integer,
        file_id -> Binary,
        tag_value_id -> Binary,
    }
}

diesel::table! {
    files (id) {
        id -> Binary,
        name -> Text,
    }
}

diesel::table! {
    tag_values (id) {
        id -> Binary,
        tag_id -> Binary,
    }
}

diesel::table! {
    tags (id) {
        id -> Binary,
        name -> Text,
    }
}

diesel::joinable!(file_tags -> files (file_id));
diesel::joinable!(tag_values -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    file_tags,
    files,
    tag_values,
    tags,
);
