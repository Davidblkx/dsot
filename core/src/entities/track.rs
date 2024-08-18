use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::tracks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Track {
    pub id: Vec<u8>,
    pub name: String,
    pub file_id: Option<Vec<u8>>
}

impl_uuid_field!(Track, id);
impl_opt_uuid_field!(Track, file_id);

impl Track {
    pub fn new(name: impl ToString) -> Self {
        Track {
            id: super::utils::new_uuid(),
            name: name.to_string(),
            file_id: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_track_with_name() {
        let t = Track::new("test");

        assert_eq!(t.name, "test");
    }

    #[test]
    fn create_track_with_id() {
        let t = Track::new("test");

        t.get_id_uuid().expect("id is not a valid UUID");
    }

    #[test]
    fn create_track_with_file_id() {
        let file_id = super::super::utils::new_uuid();
        let t = Track {
            id: super::super::utils::new_uuid(),
            name: "test".to_string(),
            file_id: Some(file_id.clone())
        };

        assert_eq!(t.file_id, Some(file_id));
        t.get_file_id_uuid().expect("file_id is not a valid UUID");
    }
}
