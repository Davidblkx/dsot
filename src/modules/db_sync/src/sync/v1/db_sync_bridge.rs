use super::handler::SyncBridge;
use super::model::*;
use crate::{DsotDatabase, RepositoryRegistry, Result, database::DsotDatabaseTransaction};

pub struct DatabaseSyncBridge<'a> {
    pub db: &'a DsotDatabase,
    pub trx: DsotDatabaseTransaction<'a>,
    should_commit: bool,
}

impl<'a> DatabaseSyncBridge<'a> {
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

    fn gen_ack_message(&self) -> Result<Handshake> {
        Ok(Handshake::Ack(self.trx.generate_sync_hash()?))
    }

    fn validate_hash(&self, remote_hash: &SyncHash) -> Result<HandshakeMessage> {
        let hash = self.trx.generate_sync_hash()?;
        if &hash == remote_hash {
            Ok(HandshakeMessage::Complete)
        } else {
            Ok(Handshake::Ack(hash).to_message())
        }
    }

    fn begin_data_exchange(&self, remote_hash: &SyncHash) -> Result<DataExchangeMessage> {
        let hash = self.trx.generate_sync_hash()?;
        if &hash == remote_hash {
            Ok(DataExchangeMessage::Complete)
        } else {
            let keys = self.trx.get_journal_keys()?;
            Ok(DataExchange::Begin(keys).to_message())
        }
    }

    async fn exchange_data(&mut self, msg: &DataExchange) -> Result<DataExchangeMessage> {
        match msg {
            DataExchange::Begin(remote_keys) => {
                let to_request = self.trx.get_keys_not_in_journal(remote_keys)?;

                Ok(DataExchange::Trade {
                    keys: self.trx.get_journal_keys()?,
                    request: to_request,
                    entries: vec![],
                }
                .to_message())
            }
            DataExchange::Validate(remote_hash) => {
                let hash = self.trx.generate_sync_hash()?;
                if remote_hash == &hash {
                    Ok(DataExchangeMessage::Complete)
                } else {
                    Ok(DataExchange::Begin(self.trx.get_journal_keys()?).to_message())
                }
            }
            DataExchange::Trade {
                keys,
                request,
                entries,
            } => {
                // Insert new entries
                let entries_to_insert: Vec<&[u8]> = entries.iter().map(|v| v.as_slice()).collect();
                RepositoryRegistry::instance()
                    .apply(&mut self.trx, &entries_to_insert)
                    .await?;
                self.should_commit = true;

                // Lookup requested entries to send back
                let entries_to_send = self.trx.get_journal_entries_in_array(&request)?;
                // Lookup keys that are not in the journal (need to be requested)
                let keys_to_request = self.trx.get_keys_not_in_journal(&keys)?;

                if entries_to_send.is_empty() && keys_to_request.is_empty() {
                    Ok(DataExchange::Validate(self.trx.generate_sync_hash()?).to_message())
                } else {
                    Ok(DataExchange::Trade {
                        keys: self.trx.get_journal_keys()?,
                        request: keys_to_request,
                        entries: entries_to_send,
                    }
                    .to_message())
                }
            }
        }
    }
}

impl<'a> SyncBridge for DatabaseSyncBridge<'a> {
    async fn read_handshake(&mut self) -> HandshakeMessage {
        HandshakeMessage::InProgress(Handshake::Hello(self.db.id.clone()))
    }

    async fn send_handshake(&mut self, msg: &HandshakeMessage) -> HandshakeMessage {
        let handshake = match msg {
            HandshakeMessage::InProgress(h) => h,
            SyncMessage::Complete => {
                return HandshakeMessage::Complete;
            }
            SyncMessage::Error(e) => {
                ::log::warn!("Handshake error: {:?}", e);
                return HandshakeMessage::Complete;
            }
        };

        match handshake {
            Handshake::Hello(id) => {
                if id == &self.db.id {
                    self.gen_ack_message().to_message()
                } else {
                    HandshakeMessage::error("Id don't match current data")
                }
            }
            Handshake::Ack(hash) => self.validate_hash(hash).flat(),
        }
    }

    async fn complete_handshake(&mut self, msg: &HandshakeMessage) -> DataExchangeMessage {
        match msg {
            HandshakeMessage::InProgress(handshake) => match handshake {
                Handshake::Ack(hash) => self.begin_data_exchange(hash).flat(),
                _ => DataExchangeMessage::error("Unexpected handshake message"),
            },
            SyncMessage::Complete => DataExchangeMessage::Complete,
            SyncMessage::Error(e) => {
                ::log::warn!("Handshake error: {:?}", e);
                DataExchangeMessage::Complete
            }
        }
    }

    async fn send_data(&mut self, msg: &super::model::DataExchangeMessage) -> DataExchangeMessage {
        match msg {
            DataExchangeMessage::InProgress(data) => self.exchange_data(data).await.flat(),
            SyncMessage::Complete => DataExchangeMessage::Complete,
            SyncMessage::Error(e) => {
                ::log::warn!("Handshake error: {:?}", e);
                self.should_commit = false;
                DataExchangeMessage::Complete
            }
        }
    }
}
