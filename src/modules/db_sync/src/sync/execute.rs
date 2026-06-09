use crate::{
    DsotDatabase, RepositoryRegistry,
    database::{DsotDatabaseError, Result},
    sync::handler::SyncMessage::Exchange,
};

use super::handler::{SyncHandler, SyncMessage};

impl DsotDatabase {
    pub async fn start_remote_sync<T: SyncHandler>(&self, handler: &T) -> Result<()> {
        log::info!("Starting sync {} width {}", self.id, handler.name());
        let need_sync = handler
            .handshake(self.id.clone(), self.generate_sync_hash()?)
            .await;

        if !need_sync {
            log::info!("No sync needed");
            return Ok(());
        }

        let mut req = SyncMessage::Exchange {
            request_entries: vec![],
            available_keys: self.get_journal_keys()?,
            requested_entries: vec![],
        };

        let trx = self.begin_transaction().await?;

        while handler.is_open() {
            match handler.sync(&req).await {
                Exchange {
                    request_entries,
                    available_keys,
                    requested_entries,
                } => {
                    // Insert new entries
                    let entries_to_insert: Vec<&[u8]> =
                        requested_entries.iter().map(|v| v.as_slice()).collect();
                    RepositoryRegistry::instance()
                        .apply(&trx, &entries_to_insert)
                        .await?;

                    // Lookup requested entries to send back
                    let entries_to_send = self.get_journal_entries_in_array(&request_entries)?;
                    // Lookup keys that are not in the journal (need to be requested)
                    let keys_to_request = self.get_keys_not_in_journal(&available_keys)?;

                    if entries_to_send.is_empty() && keys_to_request.is_empty() {
                        // All entries are already in the journal, no need to request more
                        req = SyncMessage::Complete;
                    } else {
                        req = SyncMessage::Exchange {
                            request_entries: keys_to_request,
                            available_keys: self.get_journal_keys()?,
                            requested_entries: entries_to_send,
                        };
                    }
                }
                SyncMessage::Complete => {
                    trx.commit().await?;
                    return Ok(());
                }
                SyncMessage::Fail(err) => return Err(DsotDatabaseError::SyncError(err)),
            }
        }

        trx.rollback().await?;

        return Err(DsotDatabaseError::SyncError(
            "Connection was closed".to_string(),
        ));
    }
}
