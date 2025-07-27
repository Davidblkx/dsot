use uuid::Uuid;

use crate::storage::sql::SqlOperation;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum JournalOperation {
    SQL(SqlOperation),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JournalEntryV0 {
    /// The unique identifier of the journal entry.
    pub id: Uuid,
    /// The action to be performed.
    pub op: JournalOperation,
}

crate::dsot_storage_declare_model!(JournalEntry { 0: JournalEntryV0 });

crate::dsot_storage_use_id_uuid!(JournalEntry, "journal_entry");

impl JournalEntry {
    pub fn new(op: JournalOperation) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            op,
        }
    }
}
