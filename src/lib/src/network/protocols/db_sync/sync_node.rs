use dsot_db_sync::sync::{DBSyncMessage, SyncNode};
use iroh::endpoint::Connection;

use crate::{
    error::{IntoDBSyncResult, Result},
    network::sink::NetworkChannel,
};

/// Bridge sync messages between two nodes using IROH network
///
/// It implements `dsot_db_sync::sync::v1::handler::SyncNode`
///
/// - It can be used as an active bridge by creating an instance with `start_sync` and the database id to sync
/// - It can be a passive bridge by creating an instance with `await_sync` and waiting for a connection
#[derive(Debug)]
pub struct NetworkSyncNode {
    pub channel: NetworkChannel,
    id: Option<String>,
}

impl NetworkSyncNode {
    /// Creates a new active sync node, meaning that is expect to be the one sending the handshake message
    pub async fn start_sync(connection: Connection, id: String) -> Result<Self> {
        let channel = NetworkChannel::start::<()>(connection, &None).await?;
        Ok(Self {
            channel,
            id: Some(id),
        })
    }

    /// Creates a new passive sync node, meaning that is expect to be the one receiving the handshake message
    pub async fn await_sync(connection: Connection) -> Result<Self> {
        let channel = NetworkChannel::open(connection).await?;
        Ok(Self { channel, id: None })
    }

    async fn send(&mut self, message: &DBSyncMessage) -> dsot_db_sync::Result<()> {
        self.channel.write(message).await.into_db_sync()?;
        Ok(())
    }

    async fn read(&mut self) -> dsot_db_sync::Result<DBSyncMessage> {
        match self.channel.read::<DBSyncMessage>().await {
            Ok(msg) => msg.ok().into_db_sync(),
            Err(err) => Err(err).into_db_sync(),
        }
    }
}

impl SyncNode for NetworkSyncNode {
    async fn get_db_id(&mut self) -> Option<String> {
        if let Some(db_id) = &self.id {
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

    async fn handle(&mut self, message: &DBSyncMessage) -> dsot_db_sync::Result<DBSyncMessage> {
        self.send(message).await?;
        self.read().await
    }
}
