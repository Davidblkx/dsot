use std::borrow::Borrow;
use native_model::Model;
use redb::{Key, ReadableTableMetadata, TableDefinition};

use crate::error::Result;

use super::DsotDB;
use super::entity_key::{EntityKey, KeyValue};

impl DsotDB {
    pub fn list_entities<T: Model>(&self, table: &str) -> Result<()> {
        Ok(())
    }

    pub fn count(&self, table: &str) -> Result<u64> {
        let table_def: TableDefinition<&str, Vec<u8>> = TableDefinition::new(table);

        let db_result = self.execute(|db| {
            let read_trx = db.begin_read()?;
            let table_read = read_trx.open_table(table_def)?;
            let count = table_read.len()?;
            Ok(count)
        })?;

        Ok(db_result)
    }
}
