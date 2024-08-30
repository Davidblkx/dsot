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

impl Artist {
    pub fn new(name: &str) -> Artist {
        let mut a = Artist {
            id: crate::entities::utils::new_uuid(),
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
    fn create_artist_with_name() {
        let a = Artist::new("Test Artist");

        assert_eq!(a.name, "Test Artist");
        let aliases = a.get_aliases().unwrap();
        assert_eq!(aliases.len(), 1);
        assert!(aliases.contains("Test Artist"));
    }

    #[test]
    fn can_add_alias() {
        let mut a = Artist::new("Test Artist");

        let added = a.add_aliases("Alias".to_string()).unwrap();
        assert!(added);
        let aliases = a.get_aliases().unwrap();
        assert_eq!(aliases.len(), 2);
        assert!(aliases.contains("Test Artist"));
        assert!(aliases.contains("Alias"));
    }

    #[test]
    fn handle_uuid() {
        let a = Artist::new("Test Artist");

        a.get_id_uuid().expect("Failed to get UUID");
    }

    #[test]
    fn handle_mbid() {
        let mut a = Artist::new("Test Artist");

        assert!(a.get_mbid_uuid().unwrap().is_none());
        a.set_mbid_uuid(Some(uuid::Uuid::now_v7()));
        assert!(a.get_mbid_uuid().unwrap().is_some());
    }
}
