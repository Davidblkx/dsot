use futures_util::StreamExt;
use iroh::endpoint::{Connection, RecvStream, VarInt};
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

use crate::error::*;
use crate::sink::message::{InnerNetworkMessage, NetworkMessage};

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

    pub async fn open(connection: Connection) -> Result<Self> {
        match connection.accept_uni().await {
            Ok(stream) => Ok(Self::new(stream, connection)),
            Err(e) => Err(DsotNetworkError::IrohError(e.to_string())),
        }
    }

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

    pub async fn close(self) -> () {
        self.inner_connection
            .close(VarInt::from_u32(0), b"read completed")
    }
}
