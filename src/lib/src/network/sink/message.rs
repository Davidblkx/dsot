use dsot_serde::BinarySerde;
use tokio_util::bytes::Bytes;

use crate::error::*;

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

#[derive(Debug)]
pub enum NetworkMessage<T: serde::Serialize + serde::de::DeserializeOwned> {
    Message(T),
    Disconnect,
    Error(String),
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> NetworkMessage<T> {
    pub fn ok(self) -> Result<T> {
        match self {
            NetworkMessage::Message(value) => Ok(value),
            NetworkMessage::Disconnect => Err(DsotError::NetworkDisconnected),
            NetworkMessage::Error(err) => Err(DsotError::NetworkDeviceError(err)),
        }
    }

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

pub trait NetworkMessageUnwrap {
    type MessageType: serde::Serialize + serde::de::DeserializeOwned;

    fn unwrap_message(self) -> NetworkMessage<<Self as NetworkMessageUnwrap>::MessageType>;
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> NetworkMessageUnwrap
    for Result<NetworkMessage<T>>
{
    type MessageType = T;

    fn unwrap_message(self) -> NetworkMessage<T> {
        match self {
            Ok(msg) => msg,
            Err(err) => NetworkMessage::Error(err.to_string()),
        }
    }
}
