use futures_util::StreamExt;
use iroh::endpoint::{Connection, RecvStream, VarInt};
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

use super::message::{InnerNetworkMessage, NetworkMessage};
use crate::error::*;

/// One-way communication helper, used to ensure full messages are received
#[derive(Debug)]
pub struct NetworkReader {
    pub inner_reader: FramedRead<RecvStream, LengthDelimitedCodec>,
    pub inner_connection: Connection,
}

impl NetworkReader {
    pub fn new(stream: RecvStream, connection: Connection) -> Self {
        Self {
            inner_reader: FramedRead::new(stream, LengthDelimitedCodec::new()),
            inner_connection: connection,
        }
    }

    /// Creates new reader from a connection
    pub async fn open(connection: Connection) -> Result<Self> {
        match connection.accept_uni().await {
            Ok(stream) => Ok(Self::new(stream, connection)),
            Err(e) => Err(DsotError::IrohError(e.to_string())),
        }
    }

    /// Read a message
    pub async fn read<T: serde::Serialize + serde::de::DeserializeOwned>(
        &mut self,
    ) -> Result<NetworkMessage<T>> {
        if let Some(bytes) = self.inner_reader.next().await {
            let inner = InnerNetworkMessage::from_bytes(&bytes?.iter().as_slice())?;
            inner.try_into()
        } else {
            Ok(NetworkMessage::Disconnect)
        }
    }

    /// Sends a close request cancelling and pending message to be sent/received by the connection
    pub async fn close(self) -> () {
        self.inner_connection
            .close(VarInt::from_u32(0), b"read completed")
    }
}
