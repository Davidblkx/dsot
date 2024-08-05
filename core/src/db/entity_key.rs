pub enum KeyValue {
    String(String),
    Uuid([u8; 16]),
}

pub struct EntityKey {
    pub table: String,
    pub id: KeyValue,
}

impl EntityKey {
    pub fn new(table: &str, id: KeyValue) -> Self {
        Self { table: table.to_string(), id }
    }

    pub fn create_uuid(table: &str) -> Self {
        Self {
            table: table.to_string(),
            id: KeyValue::Uuid(uuid7::uuid7().as_bytes().clone()),
        }
    }

    pub fn for_uuid(table: &str, id: &uuid7::Uuid) -> Self {
        Self {
            table: table.to_string(),
            id: KeyValue::Uuid(id.as_bytes().clone()),
        }
    }
}