use iroh::endpoint::{Connection, RecvStream, SendStream};

use crate::error::*;

use super::message::NetworkMessage;
use super::reader::NetworkReader;
use super::writer::NetworkWriter;

#[derive(Debug)]
pub struct NetworkChannel {
    pub reader: NetworkReader,
    pub writer: NetworkWriter,
}

impl NetworkChannel {
    pub fn new(read_stream: RecvStream, send_stream: SendStream, connection: Connection) -> Self {
        Self {
            reader: NetworkReader::new(read_stream, connection.clone()),
            writer: NetworkWriter::new(send_stream, connection),
        }
    }

    /// Opens a new network channel on the given connection ready to receive messages and send replies.
    pub async fn open(connection: Connection) -> Result<Self> {
        match connection.accept_bi().await {
            Ok((send, read)) => Ok(Self::new(read, send, connection)),
            Err(e) => Err(DsotError::IrohError(e.to_string())),
        }
    }

    /// Opens a new network channel on the given connection ready to send a message and await for replies
    /// If `init` is `Some`, the initial message will be sent before returning.
    pub async fn start<T: serde::Serialize + serde::de::DeserializeOwned>(
        connection: Connection,
        init: &Option<T>,
    ) -> Result<Self> {
        let mut channel = match connection.open_bi().await {
            Ok((send, read)) => Self::new(read, send, connection),
            Err(e) => return Err(DsotError::IrohError(e.to_string())),
        };

        if let Some(init) = init {
            channel.write(init).await?;
        }

        Ok(channel)
    }

    pub async fn write<T: serde::Serialize + serde::de::DeserializeOwned>(
        &mut self,
        data: &T,
    ) -> Result<()> {
        self.writer.write(data).await?;
        Ok(())
    }

    pub async fn write_error<T: ToString>(&mut self, error: T) -> Result<()> {
        self.writer.write_error(error).await?;
        Ok(())
    }

    pub async fn read<T: serde::Serialize + serde::de::DeserializeOwned>(
        &mut self,
    ) -> Result<NetworkMessage<T>> {
        Ok(self.reader.read().await?)
    }

    pub async fn close(self) -> Result<()> {
        self.writer.close().await?;
        Ok(())
    }

    pub async fn force_close(self) -> () {
        self.reader.close().await;
    }
}
