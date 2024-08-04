use redb::Database;

use crate::error::{Result, DsotError};

/// A wrapper around a `Database` that ensures that only one operation is performed at a time.
/// This is necessary because `Database` is not thread-safe.
/// 
/// # Example
/// 
/// ```
/// use dsot_core::db::DsotDB;
/// 
/// let db = DsotDB::new("test.db");
/// 
/// const TABLE: TableDefinition<&str, u64> = TableDefinition::new("my_data");
/// 
/// db.execute(|db| {
///     let write_txn = db.begin_write()?;
///     {
///         let mut table = write_txn.open_table(TABLE)?;
///         table.insert("my_key", &123)?;
///     }
///     write_txn.commit()?;
///     ok(())
/// })?;
/// ```
pub struct DsotDB {
    path: String,
    inner: std::sync::Mutex<()>,
}

impl DsotDB {
    /// Create a new thread safe database wrapper that will use the given path.
    pub fn new(path: &str) -> Self {
        DsotDB {
            inner: std::sync::Mutex::new(()),
            path: path.to_string(),
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Execute the given function with a `Database` instance.
    /// 
    /// # Example
    /// 
    /// ```
    /// use dsot::db::DsotDB;
    /// 
    /// let db = DsotDB::new("test.db");
    /// 
    /// db.execute(|db| {
    ///     let write_txn = db.begin_write()?;
    ///     {
    ///         let mut table = write_txn.open_table(TABLE)?;
    ///         table.insert("my_key", &123)?;
    ///     }
    ///     write_txn.commit()?;
    ///     ok(())
    /// })?;
    /// ```
    pub fn execute<T>(&self, f: impl Fn(Database) -> Result<T>) -> Result<T> {
        let lock = self.inner.lock().map_err(|_| DsotError::DatabaseLockError)?;

        let db = Database::create(&self.path)?;
        let result = f(db);
        drop(lock);

        result
    }
}
