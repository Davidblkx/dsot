use std::collections::HashSet;

use diesel::prelude::*;

use crate::error::Result;

pub fn create_new_album(
    db: &mut SqliteConnection,
    name: &str,
    mbid: Option<uuid::Uuid>,
    aliases: Option<Vec<String>>,
) -> Result<uuid::Uuid> {
    let mut album = super::Album::new(&name);

    let id = album.get_id_uuid()?;

    if mbid.is_some() {
        album.set_mbid_uuid(mbid);
    }

    // If aliases are provided, create HashSet and set it
    if let Some(aliases) = aliases {
        let set: HashSet<String> = aliases.into_iter().collect();
        album.set_aliases(set)?;
    }

    diesel::insert_into(crate::db::schema::albums::table)
        .values(&album)
        .execute(db)?;

    Ok(id)
}
