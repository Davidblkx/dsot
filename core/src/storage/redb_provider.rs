
use redb::ReadableTable;

use crate::error::{DsotError, Result};
use super::{StorageProvider, Storage, TableTransaction};

pub struct RedbTableTransaction<'a> {
    transaction: redb::WriteTransaction,
    table_name: &'a str,
}

impl<'a> RedbTableTransaction<'a> {
    pub fn open_table_transaction(table: &'a str, db: &redb::Database) -> Result<Self> {
        let t = db.begin_write().map_err(|e|
            DsotError::TableTransactionError("Redb".to_string(), table.to_string(), e.to_string()))?;

        Ok(RedbTableTransaction {
            transaction: t,
            table_name: table,
        })
    }

    fn get_table_definition(&self) -> redb::TableDefinition<'a, &'static [u8], &'static [u8]> {
        redb::TableDefinition::new(self.table_name)
    }
}

impl<'a> TableTransaction<'a> for RedbTableTransaction<'a> {

    fn get(&self, key: &[u8]) -> Result<Option<&'a [u8]>> {
        let def = self.get_table_definition();
        let table = self.transaction.open_table(self.get_table_definition())
            .map_err(|e| DsotError::TableTransactionError("Redb".to_string(), self.table_name.to_string(), e.to_string()))?;

        match table.get(key) {
            Ok(Some(v)) => Ok(Some(v.value())),
            Ok(None) => Ok(None),
            Err(e) => Err(DsotError::TableTransactionError("Redb".to_string(), self.table_name.to_string(), e.to_string())),
        }
    }
}
