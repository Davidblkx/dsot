use super::StorageEntry;
use crate::error::Result;

/// Represents the result of a migration.
///
/// If a migration was required, the value field will contain the new value of the entry.
///
/// # Arguments
/// * `value` - The new value of the entry, if a migration was required.
/// * `version` - The current version of the entry after the migration.
pub struct MigrationResult {
    pub value: Option<Vec<u8>>,
    pub version: u64,
}

pub trait Migration {
    type PrevVersion;

    /// Migrate the given entry.
    ///
    /// # Arguments
    /// * `entry` - The entry to migrate.
    fn migrate(entry: &StorageEntry) -> Result<MigrationResult>;

    /// Check if the given entry needs a migration.
    fn needs_migration(entry: &StorageEntry) -> Result<bool>;
}
