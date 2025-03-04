use uuid::Uuid;

use crate::error::Result;
use crate::model::{Entity, JournalEntry, JournalEntryAction};
use crate::storage::BinModel;

// WAY TOO MUCH CODE -> We need a macro to create an entity and a way to update some properties


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArtistV0 {
    pub id: Uuid,
    pub name: String,
}

crate::dsot_storage_declare_model!(Artist {
    0: ArtistV0
});

crate::dsot_storage_use_id_uuid!(Artist, "artist");

impl Artist {
    pub fn create(artist: Artist) -> Result<JournalEntry> {
        let ser = artist.serialize()?;

        Ok(JournalEntry {
            id: Uuid::now_v7(),
            entity: Entity::Artist.to_id(),
            action: JournalEntryAction::Create(ser),
            target_id: artist.id,
        })
    }
}
