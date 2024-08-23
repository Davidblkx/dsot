use std::collections::HashSet;

use diesel::prelude::*;

use crate::error::Result;

pub fn create_new_artist(
    db: &mut SqliteConnection,
    name: &str,
    mbid: Option<uuid::Uuid>,
    aliases: Option<Vec<String>>,
) -> Result<Vec<u8>> {
    let mut artist = super::Artist::new(&name);

    let id = artist.id.clone();

    if mbid.is_some() {
        artist.set_mbid_uuid(mbid);
    }

    // If aliases are provided, create HashSet and set it
    if let Some(aliases) = aliases {
        let set: HashSet<String> = aliases.into_iter().collect();
        artist.set_aliases(set)?;
    }

    diesel::insert_into(crate::db::schema::artists::table)
        .values(&artist)
        .execute(db)?;

    Ok(id)
}
