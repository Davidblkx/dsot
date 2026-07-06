use dsot_db_sync::{error::*, sync::*};
use iroh::endpoint::Connection;

use crate::sink::*;

trait ToDBSyncError {
    fn to_db_sync_error(self) -> DBSyncError;
}

impl ToDBSyncError for crate::DsotNetworkError {
    fn to_db_sync_error(self) -> DBSyncError {
        match self {
            crate::DsotNetworkError::DBSyncError(err) => err,
            err => DBSyncError::CommunicationError(err.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct NetworkDBSyncNode {
    pub channel: NetworkChannel,
    pub db_id: Option<String>,
}

impl NetworkDBSyncNode {
    pub async fn start_sync(connection: Connection, db_id: Option<String>) -> crate::Result<Self> {
        // If id is None, we open a connection waiting for remote to send the id
        let channel = if db_id.is_none() {
            NetworkChannel::open(connection).await?
        } else {
            NetworkChannel::start::<String>(connection, &None).await?
        };

        Ok(Self { channel, db_id })
    }

    pub async fn send(&mut self, msg: &DBSyncMessage) -> Result<()> {
        self.channel.write(msg).await.map_err(|e| {
            ::log::error!("[NetworkDBSyncNode] Network write error: {}", e);
            e.to_db_sync_error()
        })
    }

    pub async fn read(&mut self) -> Result<DBSyncMessage> {
        match self.channel.read().await.map_err(|e| {
            ::log::error!("[NetworkDBSyncNode] Network read error: {}", e);
            e.to_db_sync_error()
        })? {
            NetworkMessage::Message(msg) => Ok(msg),
            NetworkMessage::Disconnect => {
                Err(DBSyncError::CommunicationError("Disconnected".to_string()))
            }
            NetworkMessage::Error(err) => Err(DBSyncError::CommunicationError(err)),
        }
    }
}

impl SyncNode for NetworkDBSyncNode {
    async fn get_db_id(&mut self) -> Option<String> {
        if let Some(db_id) = &self.db_id {
            Some(db_id.clone())
        } else {
            match self.read().await {
                Ok(msg) => {
                    if let DBSyncMessage::Hello(db_id) = msg {
                        Some(db_id)
                    } else {
                        None
                    }
                }
                Err(err) => {
                    ::log::error!("[NetworkDBSyncNode] Get db id error: {}", err);
                    None
                }
            }
        }
    }

    async fn handle(&mut self, message: &DBSyncMessage) -> Result<DBSyncMessage> {
        self.send(message).await?;
        self.read().await
    }
}
