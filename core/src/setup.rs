use diesel::prelude::*;

pub fn setup() {
    use crate::db::schema::tags::dsl::*;

    println!("Setting up...");
}
