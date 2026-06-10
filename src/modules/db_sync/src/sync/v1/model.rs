pub type SyncHash = [u8; 32];
pub type SyncKey = [u8; 16];
pub type SyncEntry = Vec<u8>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SyncMessage<T> {
    Continue(T),
    Complete,
    Error(Option<String>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Handshake {
    Hello(String),
    Ack(SyncHash),
}

pub type HandshakeMessage = SyncMessage<Handshake>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Exchange {
    Begin(Vec<SyncKey>),
    Trade {
        keys: Vec<SyncKey>,
        request: Vec<SyncKey>,
        entries: Vec<SyncEntry>,
    },
    Validate(SyncHash),
}
