use crate::{
    DsotDatabase, RepositoryRegistry,
    database::{DsotDatabaseError, DsotDatabaseTransaction, Result},
    sync::{Handshake, SyncMessage},
};

use super::handler::SyncHandler;

impl DsotDatabase {
    pub fn create_handshake(&self) -> Result<Handshake> {
        Ok(Handshake {
            id: self.id.clone(),
            hash: self.generate_sync_hash()?,
        })
    }

    pub async fn start_remote_sync<T: SyncHandler>(&self, handler: &T) -> Result<()> {
        log::info!("Starting sync {} width {}", self.id, handler.name());

        let res = handler.handshake(&self.create_handshake()?).await;

        if !res.need_sync {
            log::info!("No sync needed");
            return Ok(());
        }

        let mut req = SyncMessage::Start(self.get_journal_keys()?);

        let mut trx = self.begin_transaction().await?;

        while handler.is_open() {
            println!("Sending to remote: {}", req.to_string());
            let response = handler.sync(&req).await;

            println!("Received from remote: {}", response.to_string());
            if matches!(response, SyncMessage::Complete) {
                trx.commit().await?;
                return Ok(());
            }

            req = trx.remote_sync(&response).await?;
        }

        trx.rollback().await?;

        return Err(DsotDatabaseError::SyncError(
            "Connection was closed".to_string(),
        ));
    }
}

impl<'b> DsotDatabaseTransaction<'b> {
    pub async fn remote_sync(&mut self, message: &SyncMessage) -> Result<SyncMessage> {
        match message {
            SyncMessage::Start(available_keys) => {
                // Lookup keys that are not in the journal (need to be requested)
                let keys_to_request = self.get_keys_not_in_journal(&available_keys)?;

                Ok(SyncMessage::Exchange {
                    request_entries: keys_to_request,
                    available_keys: self.get_journal_keys()?,
                    requested_entries: vec![],
                })
            }
            SyncMessage::Exchange {
                request_entries,
                available_keys,
                requested_entries,
            } => {
                // Insert new entries
                let entries_to_insert: Vec<&[u8]> =
                    requested_entries.iter().map(|v| v.as_slice()).collect();
                RepositoryRegistry::instance()
                    .apply(self, &entries_to_insert)
                    .await?;

                // Lookup requested entries to send back
                let entries_to_send = self.get_journal_entries_in_array(&request_entries)?;
                // Lookup keys that are not in the journal (need to be requested)
                let keys_to_request = self.get_keys_not_in_journal(&available_keys)?;

                if entries_to_send.is_empty() && keys_to_request.is_empty() {
                    // All entries are already in the journal, no need to request more
                    Ok(SyncMessage::Complete)
                } else {
                    Ok(SyncMessage::Exchange {
                        request_entries: keys_to_request,
                        available_keys: self.get_journal_keys()?,
                        requested_entries: entries_to_send,
                    })
                }
            }
            SyncMessage::Complete => Ok(SyncMessage::Complete),
            SyncMessage::Fail(err) => Err(DsotDatabaseError::SyncError(err.clone())),
        }
    }
}
