use redb::{ReadableDatabase, TableDefinition};
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
