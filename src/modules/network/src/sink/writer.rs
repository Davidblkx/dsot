use dsot_serde::BinarySerde;
use futures_util::SinkExt;
use iroh::endpoint::{Connection, SendStream};
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

use crate::error::*;
use crate::sink::message::InnerNetworkMessage;

#[derive(Debug)]
pub struct NetworkWriter {
    pub inner_writer: FramedWrite<SendStream, LengthDelimitedCodec>,
    pub connection: Connection,
}

impl NetworkWriter {
    pub fn new(stream: SendStream, connection: Connection) -> Self {
        Self {
            inner_writer: FramedWrite::new(stream, LengthDelimitedCodec::new()),
            connection,
        }
    }

    pub async fn open(connection: Connection) -> Result<Self> {
        match connection.open_uni().await {
            Ok(stream) => Ok(Self::new(stream, connection)),
            Err(e) => Err(DsotNetworkError::IrohError(e.to_string())),
        }
    }

    pub async fn send<T: serde::Serialize + serde::de::DeserializeOwned>(
        connection: Connection,
        data: &T,
    ) -> Result<()> {
        let mut writer = Self::open(connection).await?;
        writer.write(data).await?;
        writer.close().await?;
        Ok(())
    }

    pub async fn write<T: serde::Serialize + serde::de::DeserializeOwned>(
        &mut self,
        data: &T,
    ) -> Result<()> {
        let data_bytes = BinarySerde::serialize(data)?;
        let message = InnerNetworkMessage::Message(data_bytes).to_network_bytes()?;
        self.inner_writer.send(message).await?;
        Ok(())
    }

    pub async fn write_error<T: ToString>(&mut self, error: T) -> Result<()> {
        let message = InnerNetworkMessage::Error(error.to_string()).to_network_bytes()?;
        self.inner_writer.send(message).await?;
        Ok(())
    }

    pub async fn close(mut self) -> Result<()> {
        let message = InnerNetworkMessage::Disconnect.to_network_bytes()?;

        let _ = self.inner_writer.send(message).await;
        let _ = self.connection.closed();

        Ok(())
    }
}
