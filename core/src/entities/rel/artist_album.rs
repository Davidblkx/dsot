use diesel::prelude::*;

use crate::error::Result;

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::artist_albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ArtistAlbumRelation<'a> {
    pub artist_id: &'a Vec<u8>,
    pub album_id: &'a Vec<u8>,
}

impl ArtistAlbumRelation<'_> {
    pub fn new<'a>(artist_id: &'a Vec<u8>, album_id: &'a Vec<u8>) -> ArtistAlbumRelation<'a> {
        ArtistAlbumRelation {
            artist_id,
            album_id,
        }
    }

    pub fn create_if_new(&self, conn: &mut SqliteConnection) -> Result<bool> {
        use crate::db::schema::artist_albums::dsl::*;

        let exists = artist_albums
            .filter(artist_id.eq(self.artist_id))
            .filter(album_id.eq(self.album_id))
            .count()
            .get_result::<i64>(conn)?;

        if exists > 0 {
            return Ok(false);
        }

        diesel::insert_into(artist_albums)
            .values(self)
            .execute(conn)?;

        Ok(true)
    }
}
