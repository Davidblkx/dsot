use redb::{ReadableDatabase, ReadableTable, TableDefinition};
use rustc_hash::FxHashSet;
use uuid::Uuid;

use super::{DsotDatabase, Result};

pub const JOURNAL_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("JOURNAL");

impl DsotDatabase {
    pub fn generate_sync_hash(&self) -> Result<[u8; 32]> {
        let trx = self.journal.begin_read()?;
        let table = trx.open_table(JOURNAL_TABLE)?;
        let mut hasher = blake3::Hasher::new();
        let range = table.range::<[u8; 16]>(..)?;

        for result in range.into_iter() {
            let (key, _value) = result?;
            hasher.update(key.value().as_slice());
        }
        let finalized = hasher.finalize();

        Ok(finalized.into())
    }

    /// Returns all keys currently stored in the journal.
    pub fn get_journal_keys(&self) -> Result<Vec<[u8; 16]>> {
        let reader = self.journal.begin_read()?;
        let table = reader.open_table(JOURNAL_TABLE)?;
        let mut keys = Vec::new();

        for item in table.iter()? {
            let (key, _) = item?;
            keys.push(key.value());
        }

        Ok(keys)
    }

    /// Returns all keys that are not currently stored in the journal.
    pub fn get_keys_not_in_journal(&self, keys: &[[u8; 16]]) -> Result<Vec<[u8; 16]>> {
        let mut missing_keys = Vec::new();

        let reader = self.journal.begin_read()?;
        let table = reader.open_table(JOURNAL_TABLE)?;

        for k in keys {
            if table.get(k)?.is_none() {
                missing_keys.push(*k);
            }
        }

        Ok(missing_keys)
    }

    /// Returns all journal entries that are not in the key list.
    pub fn get_journal_entries_not_in_array(&self, keys: &[[u8; 16]]) -> Result<Vec<Vec<u8>>> {
        let mut missing_entries = Vec::new();
        let set: FxHashSet<[u8; 16]> = keys.iter().copied().collect();

        let reader = self.journal.begin_read()?;
        let table = reader.open_table(JOURNAL_TABLE)?;

        for item in table.iter()? {
            let (k, v) = item?;
            if !set.contains(&k.value()) {
                missing_entries.push(v.value().to_vec());
            }
        }

        Ok(missing_entries)
    }

    /// Returns all journal entries that are in the key list.
    pub fn get_journal_entries_in_array(&self, keys: &[[u8; 16]]) -> Result<Vec<Vec<u8>>> {
        let mut entries = Vec::new();

        let reader = self.journal.begin_read()?;
        let table = reader.open_table(JOURNAL_TABLE)?;

        for k in keys {
            if let Some(v) = table.get(k)? {
                entries.push(v.value().to_vec());
            }
        }

        Ok(entries)
    }

    pub(crate) fn remove_journal_entry(&self, id: Uuid) -> Result<()> {
        let jrn_trx = self.journal.begin_write()?;
        {
            let mut table = jrn_trx.open_table(JOURNAL_TABLE)?;
            table.remove(&id.to_bytes_le())?;
        }
        jrn_trx.commit()?;

        Ok(())
    }
}
