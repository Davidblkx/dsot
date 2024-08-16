use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::releases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Release {
    pub id: Vec<u8>,
    pub name: String,
    pub album_id: Vec<u8>,
    pub date: i64,
    pub year: i64,
    pub tracks: Vec<u8>,
    pub mbid: Option<Vec<u8>>,
}

impl_uuid_field!(Release, id);
impl_uuid_field!(Release, album_id);
impl_vec_u64_field!(Release, tracks);
impl_opt_uuid_field!(Release, mbid);

impl Release {
    pub fn new(name: impl ToString, album_id: Vec<u8>) -> Self {
        Release {
            id: super::utils::new_uuid(),
            name: name.to_string(),
            album_id,
            date: 0,
            year: 0,
            tracks: Vec::new(),
            mbid: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_release_with_name() {
        let r = Release::new("test", super::super::utils::new_uuid());

        assert_eq!(r.name, "test");
    }

    #[test]
    fn create_release_with_album_id() {
        let album_id = super::super::utils::new_uuid();
        let r = Release::new("test", album_id.clone());

        assert_eq!(r.album_id, album_id);
        r.get_album_id_uuid().expect("album_id is not a valid UUID");
    }

    #[test]
    fn create_release_with_id() {
        let r = Release::new("test", super::super::utils::new_uuid());

        r.get_id_uuid().expect("id is not a valid UUID");
    }

    #[test]
    fn handle_mbid() {
        let mut r = Release::new("test", super::super::utils::new_uuid());

        assert!(r.get_mbid_uuid().unwrap().is_none());
        r.set_mbid_uuid(Some(uuid7::new_v7()));
        assert!(r.get_mbid_uuid().unwrap().is_some());
    }

    #[test]
    fn handle_tracks() {
        let mut r = Release::new("test", super::super::utils::new_uuid());

        assert_eq!(r.get_tracks().unwrap(), Vec::new());

        r.add_tracks(5).expect("Failed to add tracks");
        r.add_tracks(7).expect("Failed to add tracks");

        assert_eq!(r.get_tracks().unwrap(), vec![5, 7]);
    }
}
