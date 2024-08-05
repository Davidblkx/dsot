use std::borrow::Borrow;
use native_model::Model;
use redb::{Key, TableDefinition};

use crate::error::Result;

use super::DsotDB;
use super::entity_key::{EntityKey, KeyValue};

impl DsotDB {
    /// Check if an entity exists in the database.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key of the entity to check.
    /// 
    /// # Returns
    /// 
    /// `true` if the entity exists, `false` otherwise.
    pub fn has_entity(&self, key: &EntityKey) -> Result<bool> {
        let value = match &key.id {
            KeyValue::String(id) => self.read_string(&key.table, id)?,
            KeyValue::Uuid(id) => self.read_uuid(&key.table, id)?,
        };

        Ok(value.is_some())
    }

    /// Read an entity from the database.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key of the entity to read.
    /// 
    /// # Returns
    /// 
    /// A tuple containing the entity and the version of the entity.
    /// If the entity does not exist, `None` is returned.
    pub fn read_entity<T: Model>(&self, key: &EntityKey) -> Result<Option<(T, u32)>> {
        let value = match &key.id {
            KeyValue::String(id) => self.read_string(&key.table, id)?,
            KeyValue::Uuid(id) => self.read_uuid(&key.table, id)?,
        };

        match value {
            Some(value) => {
                let ent = native_model::decode::<T>(value)?;
                Ok(Some(ent))
            },
            None => Ok(None)
        }
    }

    fn read_string(&self, table: &str, id: &str) -> Result<Option<Vec<u8>>> {
        let table_def: TableDefinition<&str, Vec<u8>> = TableDefinition::new(table);
        self.get_value(table_def, id)
    }

    fn read_uuid(&self, table: &str, id: &[u8; 16]) -> Result<Option<Vec<u8>>> {
        let table_def: TableDefinition<&[u8; 16], Vec<u8>> = TableDefinition::new(table);
        self.get_value(table_def, id)
    }

    fn get_value<'a, K: Key + 'static>(&self, table: TableDefinition<'a, K, Vec<u8>>, id: impl Borrow<K::SelfType<'a>>) -> Result<Option<Vec<u8>>> {
        let db_result = self.execute(|db| {
            let read_txn = db.begin_read()?;
            let table = read_txn.open_table(table)?;
            let result = table.get(id.borrow())?;
            match result {
                Some(value) => {
                    Ok(Some(value))
                },
                None => Ok(None)
            }
        })?;

        match db_result {
            Some(value) => Ok(Some(value.value())),
            None => Ok(None)
        }
    }
}