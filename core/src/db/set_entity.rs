use std::borrow::Borrow;
use native_model::Model;
use redb::{Key, TableDefinition};

use crate::error::Result;

use super::DsotDB;
use super::entity_key::{EntityKey, KeyValue};

impl DsotDB {
    /// Insert an entity into the database.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key of the entity to insert.
    /// * `entity` - The entity to insert.
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the entity was inserted successfully.
    /// 
    pub fn insert_entity<T: Model>(&self, key: &EntityKey, entity: &T) -> Result<()> {
        let value = native_model::encode(entity)?;

        match &key.id {
            KeyValue::String(id) => self.set_string(&key.table, id, value)?,
            KeyValue::Uuid(id) => self.set_uuid(&key.table, id, value)?,
        };

        Ok(())
    }

    fn set_string(&self, table: &str, id: &str, value: Vec<u8>) -> Result<()> {
        let table_def: TableDefinition<&str, Vec<u8>> = TableDefinition::new(table);
        self.insert_value(table_def, id, value)
    }

    fn set_uuid(&self, table: &str, id: &[u8; 16], value: Vec<u8>) -> Result<()> {
        let table_def: TableDefinition<&[u8; 16], Vec<u8>> = TableDefinition::new(table);
        self.insert_value(table_def, id, value)
    }

    fn insert_value<'a, K: Key + 'static>(&self, table: TableDefinition<K, Vec<u8>>, id: impl Borrow<K::SelfType<'a>>, value: Vec<u8>) -> Result<()> {
        self.execute(|db| {
            let write_txn = db.begin_write()?;
            {
                let mut table = write_txn.open_table(table)?;
                table.insert(id.borrow(), value.borrow())?;
            }
            write_txn.commit()?;

            Ok(())
        })?;

        Ok(())
    }
}