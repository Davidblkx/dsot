use dsot_core::{storage::Storage, error::{DsotError, Result}};
use super::RedbTransaction;

static TABLE_NAME : &'static str = "table_versions";
static TABLE_VERSIONS_DEF: redb::TableDefinition<&str, u64> = redb::TableDefinition::new(TABLE_NAME);

/// Wrapper around the redb::Database struct
///
/// It allows to create transactions for the database tables.
pub struct RedbStorage {
    pub db: redb::Database,
}

impl RedbStorage {
    pub fn new(db: redb::Database) -> Self {
        Self { db }
    }
}

impl Storage for RedbStorage {
    type T = RedbTransaction;

    fn get_table_version(&self, table_name: &str) -> Result<u64> {
        let version = self.db.begin_read()
            .map_err(to_trx_err!(TABLE_NAME, "Create read transaction"))?
            .open_table(TABLE_VERSIONS_DEF)
            .map_err(to_trx_err!(TABLE_NAME, "Open table"))?
            .get(table_name)
            .map_err(to_trx_err!(TABLE_NAME, "Read table version"))?
            .map(|v| v.value())
            .unwrap_or(0);
        Ok(version)
    }

    fn set_table_version(&self, table_name: &str, version: u64) -> Result<()> {
        let trx = self.db.begin_write()
            .map_err(to_trx_err!(TABLE_NAME, "Create write transaction"))?;

        trx.open_table(TABLE_VERSIONS_DEF)
            .map_err(to_trx_err!(TABLE_NAME, "Open table"))?
            .insert(table_name, version)
            .map_err(to_trx_err!(TABLE_NAME, "Insert table version"))?;

        trx.commit()
            .map_err(to_trx_err!(TABLE_NAME, "Commit transaction"))?;

        Ok(())
    }

    fn open_table(&self, table_name: &'static str) -> Result<Self::T> {
        let trx = self.db.begin_write()
            .map_err(to_trx_err!(table_name, "Create write transaction"))?;

        Ok(RedbTransaction::new(trx, table_name))
    }
}

#[cfg(test)]
mod tests {
    use dsot_core::storage::Storage;

    use crate::storage::redb::RedbInMemoryProvider;

    static TABLE_NAME : &'static str = "TEST_TABLE";

    #[test]
    fn test_get_table_version() {
        let storage = RedbInMemoryProvider::create().unwrap();

        let version = storage.get_table_version(TABLE_NAME).unwrap();
        assert_eq!(version, 0);
    }

    #[test]
    fn test_set_table_version() {
        let storage = RedbInMemoryProvider::create().unwrap();

        storage.set_table_version(TABLE_NAME, 5).unwrap();
        let version = storage.get_table_version(TABLE_NAME).unwrap();

        assert_eq!(version, 5);
    }

    #[test]
    fn test_open_table() {
        let storage = RedbInMemoryProvider::create().unwrap();

        let trx = storage.open_table(TABLE_NAME).unwrap();
        assert_eq!(trx.name, TABLE_NAME);
    }
}
