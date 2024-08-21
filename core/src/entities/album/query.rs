use diesel::prelude::*;

use crate::error::Result;

use super::Album;

pub fn list_albums(db: &mut SqliteConnection) -> Result<Vec<Album>> {
    use crate::db::schema::albums::dsl::*;

    let results = albums.load::<Album>(db)?;

    Ok(results)
}
