use uuid::Uuid;

use crate::db::DbOperation;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum JournalOperation {
    Db(DbOperation),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JournalEntryV0 {
    /// The unique identifier of the journal entry.
    pub id: Uuid,
    /// The action to be performed.
    pub op: JournalOperation,
}

crate::dsot_storage_declare_model!(JournalEntry {
    0: JournalEntryV0
});

crate::dsot_storage_use_id_uuid!(JournalEntry, "journal_entry");
