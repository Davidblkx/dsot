use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Album {
    pub id: Vec<u8>,
    pub name: String,
    pub mbid: Option<Vec<u8>>,
    pub aliases: Vec<u8>,
}

impl_uuid_field!(Album, id);
impl_opt_uuid_field!(Album, mbid);
impl_hashset_str_field!(Album, aliases);

impl Album {
    pub fn new(name: &str) -> Album {
        let mut a = Album {
            id: super::utils::new_uuid(),
            name: name.to_string(),
            aliases: Vec::new(),
            mbid: None,
        };

        a.add_aliases(name.to_string()).unwrap();

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn create_album_with_name() {
        let a = Album::new("Test Album");

        assert_eq!(a.name, "Test Album");
        let aliases = a.get_aliases().unwrap();
        assert_eq!(aliases.len(), 1);
        assert!(aliases.contains("Test Album"));
    }

    #[test]
    fn can_add_alias() {
        let mut a = Album::new("Test Album");

        let added = a.add_aliases("Alias".to_string()).unwrap();
        assert!(added);
        let aliases = a.get_aliases().unwrap();
        assert_eq!(aliases.len(), 2);
        assert!(aliases.contains("Test Album"));
        assert!(aliases.contains("Alias"));
    }

    #[test]
    fn handle_uuid() {
        let a = Album::new("Test Album");

        a.get_id_uuid().expect("Failed to get UUID");
    }

    #[test]
    fn handle_mbid() {
        let mut a = Album::new("Test Album");

        assert!(a.get_mbid_uuid().unwrap().is_none());

        a.set_mbid_uuid(Some(uuid7::new_v7()));
        assert!(a.get_mbid_uuid().unwrap().is_some());
    }
}
