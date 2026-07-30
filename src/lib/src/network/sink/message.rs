use dsot_serde::BinarySerde;
use tokio_util::bytes::Bytes;

use crate::error::*;

/// This is the actual message sent between nodes.
///
/// Functions as an helper that sits between the `Bytes` sent and
/// the generic `NetworkMessage<T>`.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum InnerNetworkMessage {
    Message(Vec<u8>),
    Disconnect,
    Error(String),
}

impl InnerNetworkMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(BinarySerde::serialize(self)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<InnerNetworkMessage> {
        Ok(BinarySerde::deserialize(bytes)?)
    }

    pub fn to_network_bytes(self) -> Result<Bytes> {
        Ok(Bytes::from(self.to_bytes()?))
    }
}

/// Messages sent/received by the primitives `NetworkChannel`, `NetworkReader` and `NetworkWriter`
#[derive(Debug)]
pub enum NetworkMessage<T: serde::Serialize + serde::de::DeserializeOwned> {
    /// Message to send/receive
    Message(T),
    /// Disconnect signal
    Disconnect,
    /// Error
    Error(String),
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> NetworkMessage<T> {
    /// Map message to `Result<T>` meaning that it his disconnect or error, a new DsotError is returned
    pub fn ok(self) -> Result<T> {
        match self {
            NetworkMessage::Message(value) => Ok(value),
            NetworkMessage::Disconnect => Err(DsotError::NetworkDisconnected),
            NetworkMessage::Error(err) => Err(DsotError::NetworkDeviceError(err)),
        }
    }

    /// Serialize message to `Bytes`
    pub fn to_network_bytes(self) -> Result<Bytes> {
        let inner: InnerNetworkMessage = self.try_into()?;
        Ok(Bytes::from(inner.to_bytes()?))
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> TryFrom<InnerNetworkMessage>
    for NetworkMessage<T>
{
    type Error = DsotError;

    fn try_from(message: InnerNetworkMessage) -> Result<Self> {
        match message {
            InnerNetworkMessage::Message(bytes) => {
                Ok(NetworkMessage::Message(BinarySerde::deserialize(&bytes)?))
            }
            InnerNetworkMessage::Disconnect => Ok(NetworkMessage::Disconnect),
            InnerNetworkMessage::Error(err) => Ok(NetworkMessage::Error(err)),
        }
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> TryFrom<NetworkMessage<T>>
    for InnerNetworkMessage
{
    type Error = DsotError;

    fn try_from(message: NetworkMessage<T>) -> Result<Self> {
        match message {
            NetworkMessage::Message(bytes) => Ok(InnerNetworkMessage::Message(
                BinarySerde::serialize(&bytes)?,
            )),
            NetworkMessage::Disconnect => Ok(InnerNetworkMessage::Disconnect),
            NetworkMessage::Error(err) => Ok(InnerNetworkMessage::Error(err)),
        }
    }
}
