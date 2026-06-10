use crate::Result;
use crate::dser::EntityMessagePack;

pub type SyncHash = [u8; 32];
pub type SyncKey = [u8; 16];
pub type SyncEntry = Vec<u8>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SyncMessage<T> {
    InProgress(T),
    Complete,
    Error(Option<String>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Handshake {
    Hello(String),
    Ack(SyncHash),
}

impl Handshake {
    pub fn to_message(self) -> HandshakeMessage {
        HandshakeMessage::InProgress(self)
    }
}

pub type HandshakeMessage = SyncMessage<Handshake>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum DataExchange {
    Begin(Vec<SyncKey>),
    Trade {
        keys: Vec<SyncKey>,
        request: Vec<SyncKey>,
        entries: Vec<SyncEntry>,
    },
    Validate(SyncHash),
}

impl DataExchange {
    pub fn to_message(self) -> DataExchangeMessage {
        SyncMessage::InProgress(self)
    }
}

pub type DataExchangeMessage = SyncMessage<DataExchange>;

impl<T> SyncMessage<T> {
    pub fn new(value: T) -> Self {
        Self::InProgress(value)
    }

    pub fn error<E: ToString>(e: E) -> Self {
        Self::Error(Some(e.to_string()))
    }

    pub fn empty_error() -> Self {
        Self::Error(None)
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> SyncMessage<T> {
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        EntityMessagePack::serialize(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> SyncMessage<T> {
        match EntityMessagePack::deserialize::<SyncMessage<T>>(bytes) {
            Ok(value) => value,
            Err(e) => SyncMessage::error(e),
        }
    }
}

pub trait ToSyncMessage<T> {
    fn to_message(self) -> SyncMessage<T>;
}

pub trait FlatSyncMessage<T> {
    fn flat(self) -> SyncMessage<T>;
}

impl<T> ToSyncMessage<T> for Result<T> {
    fn to_message(self) -> SyncMessage<T> {
        match self {
            Ok(h) => SyncMessage::InProgress(h),
            Err(e) => {
                ::log::warn!("Sync message error: {0}", e);
                SyncMessage::error(e)
            }
        }
    }
}

impl<T> FlatSyncMessage<T> for Result<SyncMessage<T>> {
    fn flat(self) -> SyncMessage<T> {
        match self {
            Ok(h) => h,
            Err(e) => {
                ::log::warn!("Sync message error: {0}", e);
                SyncMessage::error(e)
            }
        }
    }
}
