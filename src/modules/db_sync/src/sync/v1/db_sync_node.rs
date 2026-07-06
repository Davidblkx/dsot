use super::{error::SyncError, handler::SyncNode, model::*};
use crate::{DsotDatabase, RepositoryRegistry, database::DsotDatabaseTransaction, error::*};

pub struct DatabaseSyncNode<'a> {
    pub db: &'a DsotDatabase,
    pub trx: DsotDatabaseTransaction<'a>,
    should_commit: bool,
}

impl<'a> DatabaseSyncNode<'a> {
    pub async fn create(db: &'a DsotDatabase) -> Result<Self> {
        let trx = db.begin_transaction().await?;

        Ok(Self {
            db,
            trx,
            should_commit: false,
        })
    }

    pub async fn close(self) -> Result<bool> {
        if self.should_commit {
            self.trx.commit().await?;
            Ok(true)
        } else {
            self.trx.rollback().await?;
            Ok(false)
        }
    }

    fn handshake(&self, id: &String) -> Result<DBSyncMessage> {
        if id == &self.db.id {
            Ok(DBSyncMessage::Validate(self.trx.generate_sync_hash()?))
        } else {
            Ok(SyncError::DatabaseMissmatch.to_sync_message())
        }
    }

    fn validate_hash(&self, hash: &SyncHash) -> Result<DBSyncMessage> {
        let db_hash = self.trx.generate_sync_hash()?;
        if &db_hash == hash {
            Ok(DBSyncMessage::Completed)
        } else {
            Ok(DBSyncMessage::BeginExchange(self.trx.get_journal_keys()?))
        }
    }

    fn begin_exchange(&self, remote_keys: &Vec<SyncKey>) -> Result<DBSyncMessage> {
        let missing_keys = self.trx.get_keys_not_in_journal(remote_keys)?;
        let local_keys = self.trx.get_journal_keys()?;

        Ok(DBSyncMessage::Exchange {
            keys: local_keys,
            request: missing_keys,
            entries: vec![],
        })
    }

    async fn exchange(
        &mut self,
        remote_keys: &Vec<SyncKey>,
        requested_keys: &Vec<SyncKey>,
        entries: &Vec<SyncEntry>,
    ) -> Result<DBSyncMessage> {
        let entries_to_insert: Vec<&[u8]> = entries.iter().map(|v| v.as_slice()).collect();
        if !entries_to_insert.is_empty() {
            RepositoryRegistry::instance()
                .apply(&mut self.trx, &entries_to_insert)
                .await?;
            self.should_commit = true;
        }

        // Lookup requested entries to send back
        let entries_to_send = self.trx.get_journal_entries_in_array(&requested_keys)?;
        // Lookup keys that are not in the journal (need to be requested)
        let missing_keys = self.trx.get_keys_not_in_journal(&remote_keys)?;

        if entries_to_send.is_empty() && missing_keys.is_empty() {
            Ok(DBSyncMessage::Validate(self.trx.generate_sync_hash()?))
        } else {
            Ok(DBSyncMessage::Exchange {
                keys: self.trx.get_journal_keys()?,
                request: missing_keys,
                entries: entries_to_send,
            })
        }
    }
}

impl<'a> SyncNode for DatabaseSyncNode<'a> {
    async fn get_db_id(&mut self) -> Option<String> {
        Some(self.db.id.clone())
    }

    async fn handle(&mut self, message: &DBSyncMessage) -> Result<DBSyncMessage> {
        match message {
            DBSyncMessage::Completed => Ok(DBSyncMessage::Completed),
            DBSyncMessage::Error(e) => {
                ::log::error!("Remote node sync error: {}", e.to_msg_string());
                Ok(DBSyncMessage::Completed)
            }
            DBSyncMessage::Hello(id) => self.handshake(id),
            DBSyncMessage::Validate(hash) => self.validate_hash(hash),
            DBSyncMessage::BeginExchange(remote_keys) => self.begin_exchange(remote_keys),
            DBSyncMessage::Exchange {
                keys,
                request,
                entries,
            } => self.exchange(keys, request, entries).await,
        }
    }
}
