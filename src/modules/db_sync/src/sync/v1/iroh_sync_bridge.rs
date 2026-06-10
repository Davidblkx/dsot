use futures_util::{SinkExt, StreamExt};
use iroh::endpoint::{Connection, RecvStream, SendStream};
use iroh::protocol::AcceptError;
use tokio_util::bytes::Bytes;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::DBSyncError;

use super::handler::SyncBridge;
use super::model::*;

pub struct IrohSyncBridge {
    pub reader: FramedRead<RecvStream, LengthDelimitedCodec>,
    pub writer: FramedWrite<SendStream, LengthDelimitedCodec>,
    pub id: String,
}

impl IrohSyncBridge {
    pub async fn create(conn: Connection) -> Result<Self, AcceptError> {
        let (stream_writer, stream_reader) = conn.accept_bi().await?;

        let reader = FramedRead::new(stream_reader, LengthDelimitedCodec::new());
        let writer = FramedWrite::new(stream_writer, LengthDelimitedCodec::new());

        let mut bridge = Self {
            reader,
            writer,
            id: "".to_string(),
        };

        let id = match bridge.read_handshake_from_stream().await {
            Err(e) => {
                return Err(AcceptError::from_err(e));
            }
            Ok(msg) => match msg {
                SyncMessage::InProgress(Handshake::Hello(id)) => id,
                _ => {
                    return Err(AcceptError::from_err(DBSyncError::SyncError(
                        "Invalid handshake message".to_string(),
                    )));
                }
            },
        };
        bridge.id = id;

        Ok(bridge)
    }

    pub async fn read_handshake_from_stream(&mut self) -> crate::Result<HandshakeMessage> {
        if let Some(bytes) = self.reader.next().await {
            Ok(HandshakeMessage::from_bytes(&bytes?.iter().as_slice()))
        } else {
            Ok(HandshakeMessage::error("Disconnect"))
        }
    }

    pub async fn write_handshake_to_stream(&mut self, msg: &HandshakeMessage) -> crate::Result<()> {
        let bytes = Bytes::from(msg.to_bytes()?);
        self.writer.send(bytes).await?;
        Ok(())
    }

    pub async fn read_data_from_stream(&mut self) -> crate::Result<DataExchangeMessage> {
        if let Some(bytes) = self.reader.next().await {
            Ok(DataExchangeMessage::from_bytes(&bytes?.iter().as_slice()))
        } else {
            Ok(DataExchangeMessage::error("Disconnect"))
        }
    }

    pub async fn write_data_to_stream(&mut self, msg: &DataExchangeMessage) -> crate::Result<()> {
        let bytes = Bytes::from(msg.to_bytes()?);
        self.writer.send(bytes).await?;
        Ok(())
    }
}

impl SyncBridge for IrohSyncBridge {
    async fn read_handshake(&mut self) -> HandshakeMessage {
        Handshake::Hello(self.id.clone()).to_message()
    }

    async fn send_handshake(&mut self, msg: &HandshakeMessage) -> HandshakeMessage {
        match self.write_handshake_to_stream(msg).await {
            Ok(_) => {}
            Err(e) => {
                return HandshakeMessage::error(e);
            }
        };

        self.read_handshake_from_stream().await.flat()
    }

    async fn complete_handshake(&mut self, msg: &HandshakeMessage) -> DataExchangeMessage {
        match self.write_handshake_to_stream(msg).await {
            Ok(_) => {}
            Err(e) => {
                return DataExchangeMessage::error(e);
            }
        };

        self.read_data_from_stream().await.flat()
    }

    async fn send_data(&mut self, msg: &DataExchangeMessage) -> DataExchangeMessage {
        match self.write_data_to_stream(msg).await {
            Ok(_) => {}
            Err(e) => {
                return DataExchangeMessage::error(e);
            }
        };

        self.read_data_from_stream().await.flat()
    }
}
