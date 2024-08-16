use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: Vec<u8>,
    pub name: String,
    pub type_id: Vec<u8>,
}

impl_uuid_field!(Tag, id);
impl_uuid_field!(Tag, type_id);

impl Tag {
    pub fn new(name: impl ToString, type_id: Vec<u8>) -> Self {
        Tag {
            id: super::utils::new_uuid(),
            name: name.to_string(),
            type_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tag_with_name() {
        let t = Tag::new("test", super::super::utils::new_uuid());

        assert_eq!(t.name, "test");
    }

    #[test]
    fn create_tag_with_type_id() {
        let type_id = super::super::utils::new_uuid();
        let t = Tag::new("test", type_id.clone());

        assert_eq!(t.type_id, type_id);
        t.get_type_id_uuid().expect("type_id is not a valid UUID");
    }

    #[test]
    fn create_tag_with_id() {
        let t = Tag::new("test", super::super::utils::new_uuid());

        t.get_id_uuid().expect("id is not a valid UUID");
    }
}
