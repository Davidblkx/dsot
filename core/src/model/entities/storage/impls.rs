use uuid::Uuid;

use super::Storage;

impl Storage {
    pub fn new(
        description: String,
        mount: String,
        root: String,
        serial_number: String,
        is_default: bool,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            description,
            mount,
            root,
            serial_number,
            is_default,
        }
    }
}
