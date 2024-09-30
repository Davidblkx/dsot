use crate::error::Result;
use super::StorageEntry;

pub trait StorageEntity {
    type Item;

    fn table_name() -> &'static str;
    fn version() -> u64;

    fn update_format<'a>(entry: StorageEntry<'a>) -> Result<StorageEntry<'a>>;
    fn serialize<'a>(&self) -> Result<StorageEntry<'a>>;
    fn deserialize<'a>(entry: StorageEntry<'a>) -> Result<Self::Item>;
}
