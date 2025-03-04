/// Represents a single operation that can be performed on the database.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DbOperation {
    Create {
        id: uuid::Uuid,
        entity: u32,
        data: Vec<u8>,
    },
    Update {
        id: uuid::Uuid,
        entity: u32,
        props: Vec<(String, String)>
    },
    Delete {
        id: uuid::Uuid,
        entity: u32,
    },
}
