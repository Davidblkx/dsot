use super::handler::SyncBridge;
use super::model::*;
use crate::{DsotDatabase, Result, database::DsotDatabaseTransaction};

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
        DataExchangeMessage::Complete
    }
}
