use uuid::Uuid;

use super::Storage;

impl Storage {
    pub fn new(
        kind: String
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            kind,
            description: None,
            info: None,
            path: None
        }
    }
}
