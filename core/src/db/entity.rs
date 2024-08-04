use native_model::Model;
use crate::error::Result;

use super::DsotDB;

impl DsotDB {
    pub fn read_entity<T: Model>(&self, id: &str) -> Result<Option<T>> {
        // TODO: Implement this
    }
}