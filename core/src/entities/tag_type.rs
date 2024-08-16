use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::tag_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TagType {
    pub id: Vec<u8>,
    pub name: String,
}

impl_uuid_field!(TagType, id);

impl TagType {
    pub fn new(name: impl ToString) -> Self {
        TagType {
            id: super::utils::new_uuid(),
            name: name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tag_type_with_name() {
        let t = TagType::new("test");

        assert_eq!(t.name, "test");
    }

    #[test]
    fn create_tag_type_with_id() {
        let t = TagType::new("test");

        t.get_id_uuid().expect("id is not a valid UUID");
    }
}
