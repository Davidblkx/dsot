use dsot_db_sync::sync::{
    DataExchangeMessage, Handshake, HandshakeMessage, SyncMessage, handler::SyncBridge,
};
use iroh::endpoint::Connection;

use crate::error::*;
use crate::sink::*;

#[derive(Debug)]
pub struct NetworkSyncBridge {
    pub channel: NetworkChannel,
}

impl NetworkSyncBridge {
    pub async fn start_passive_sync(connection: Connection) -> Result<Self> {
        let channel = NetworkChannel::open(connection).await?;
        Ok(Self { channel })
    }

    pub async fn start_sync(connection: Connection, msg: HandshakeMessage) -> Result<Self> {
        let channel = NetworkChannel::start(connection, &msg).await?;
        Ok(Self { channel })
    }

    pub async fn read_hello_handshake(&mut self) -> Result<String> {
        match self.read_remote_handshake().await {
            SyncMessage::Complete => Err(crate::error::DsotNetworkError::RemoteError(
                "Handshake failed".to_string(),
            )),
            SyncMessage::InProgress(handshake) => match handshake {
                Handshake::Hello(db_id) => Ok(db_id),
                Handshake::Ack(_) => Err(crate::error::DsotNetworkError::RemoteError(
                    "Handshake failed".to_string(),
                )),
            },
            SyncMessage::Error(e) => Err(crate::error::DsotNetworkError::RemoteError(
                e.unwrap_or_default(),
            )),
        }
    }

    pub async fn read_remote_handshake(&mut self) -> HandshakeMessage {
        match self
            .channel
            .read::<HandshakeMessage>()
            .await
            .unwrap_message()
        {
            NetworkMessage::Disconnect => HandshakeMessage::error("Disconnect"),
            NetworkMessage::Error(e) => HandshakeMessage::error(e),
            NetworkMessage::Message(msg) => msg,
        }
    }

    pub async fn read_data(&mut self) -> DataExchangeMessage {
        match self
            .channel
            .read::<DataExchangeMessage>()
            .await
            .unwrap_message()
        {
            NetworkMessage::Disconnect => DataExchangeMessage::error("Disconnect"),
            NetworkMessage::Error(e) => DataExchangeMessage::error(e),
            NetworkMessage::Message(msg) => msg,
        }
    }
}

impl SyncBridge for NetworkSyncBridge {
    async fn read_handshake(&mut self) -> HandshakeMessage {
        self.read_remote_handshake().await
    }

    async fn send_handshake(&mut self, msg: &HandshakeMessage) -> HandshakeMessage {
        match self.channel.write(msg).await {
            Ok(_) => {}
            Err(e) => return HandshakeMessage::error(&e.to_string()),
        };

        self.read_remote_handshake().await
    }

    async fn complete_handshake(&mut self, msg: &HandshakeMessage) -> DataExchangeMessage {
        match self.channel.write(msg).await {
            Ok(_) => {}
            Err(e) => return DataExchangeMessage::error(&e.to_string()),
        };

        self.read_data().await
    }

    async fn send_data(&mut self, msg: &DataExchangeMessage) -> DataExchangeMessage {
        match self.channel.write(msg).await {
            Ok(_) => {}
            Err(e) => return DataExchangeMessage::error(&e.to_string()),
        };

        self.read_data().await
    }
}
