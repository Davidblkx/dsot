use crate::db::{LocalDBManager, local_db::ConnectionGuard};
use crate::error::Result;

pub struct DSOTInstance {
    pub db_manager: LocalDBManager,
    pub library: String,
}

impl DSOTInstance {
    pub fn connect_library(&self) -> Result<ConnectionGuard> {
        self.db_manager.get(&self.library)
    }
}
