use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum JournalEntryAction {
    Create(Vec<u8>),
    Update(Vec<(String, String)>),
    Delete(Uuid),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JournalEntryV0 {
    /// The unique identifier of the journal entry.
    pub id: Uuid,
    /// The entity type of the journal entry. Maps to enum Entity::to_id().
    pub entity: u32,
    /// The action to be performed.
    pub action: JournalEntryAction,
    /// The target entity id of the action.
    pub target_id: Uuid,
}

crate::dsot_storage_declare_model!(JournalEntry {
    0: JournalEntryV0
});

crate::dsot_storage_use_id_uuid!(JournalEntry, "journal_entry");
