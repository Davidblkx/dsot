use diesel::prelude::*;

use crate::entities::tag::{STATIC_TAG, Tag};

pub fn setup() {
    use crate::db::schema::tags::dsl::*;

    println!("Setting up...");

    let connection = &mut crate::db::connect_db();
    let static_tag = tags
        .filter(id.eq(STATIC_TAG.as_bytes().to_vec()))
        .limit(1)
        .select(Tag::as_select())
        .load(connection)
        .expect("Error loading static tag");

    if static_tag.len() <= 0 {
        print!("Creating static tag...");

        let new_tag = Tag::for_uuid(&STATIC_TAG, "Static");
        diesel::insert_into(crate::db::schema::tags::table)
            .values(&new_tag)
            .execute(connection)
            .expect("Error inserting static tag");

        println!("done");
    }

    let all_tags = tags
        .select(Tag::as_select())
        .load(connection)
        .expect("Error loading tags");

    println!("Tags: {:?}", all_tags);
}
