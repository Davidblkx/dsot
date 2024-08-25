use chrono::Utc;
use diesel::prelude::*;

use crate::error::Result;

pub fn create_new_release(
    db: &mut SqliteConnection,
    name: &str,
    album_id: Vec<u8>,
    date: Option<&chrono::DateTime<Utc>>,
    mbid: Option<uuid::Uuid>,
) -> Result<Vec<u8>> {
    let mut release = super::Release::new(&name, album_id);

    let id = release.id.clone();

    match date {
        Some(date) => release.set_date(date),
        None => {}
    }

    if mbid.is_some() {
        release.set_mbid_uuid(mbid);
    }

    diesel::insert_into(crate::db::schema::releases::table)
        .values(&release)
        .execute(db)?;

    Ok(id)
}
