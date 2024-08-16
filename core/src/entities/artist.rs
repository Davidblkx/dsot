use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: Vec<u8>,
    pub name: String,
    pub aliases: Vec<u8>,
    pub mbid: Option<Vec<u8>>,
}

impl_uuid_field!(Artist, id);
impl_opt_uuid_field!(Artist, mbid);
impl_hashset_str_field!(Artist, aliases);
