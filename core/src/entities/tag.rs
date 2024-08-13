use diesel::prelude::*;

pub const STATIC_TAG: uuid::Uuid = uuid::uuid!("00000000-0000-7000-b000-000000000001");

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: Vec<u8>,
    pub name: String,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid7::new_v7().as_bytes().to_vec(),
            name: name.to_string(),
        }
    }

    pub fn for_uuid(id: &uuid::Uuid, name: &str) -> Self {
        Self {
            id: id.as_bytes().to_vec(),
            name: name.to_string(),
        }
    }
}

impl core::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id_bytes: &[u8; 16] = self.id.as_slice()
            .try_into()
            .expect("Invalid UUID bytes");
        let uuid = uuid::Uuid::from_bytes(id_bytes.clone());
        f.write_fmt(format_args!("Tag {{ id: {}, name: {} }}", uuid, self.name))
    }
}
