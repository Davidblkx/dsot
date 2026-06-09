use super::model::{SyncHash, SyncKey};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SyncMessage {
    Exchange {
        request_entries: Vec<SyncKey>,
        available_keys: Vec<SyncKey>,
        requested_entries: Vec<Vec<u8>>,
    },
    Complete,
    Fail(String),
}

impl SyncMessage {
    pub fn new_fail<T: ToString>(message: T) -> Self {
        Self::Fail(message.to_string())
    }
}

pub trait SyncHandler {
    fn name(&self) -> String;

    fn is_open(&self) -> bool;

    fn handshake(&self, id: String, hash: SyncHash) -> impl Future<Output = bool>;

    fn sync(&self, state: &SyncMessage) -> impl Future<Output = SyncMessage>;
}
