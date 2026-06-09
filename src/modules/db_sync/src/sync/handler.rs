use super::model::{SyncHash, SyncMessage};

pub trait SyncHandler {
    fn name(&self) -> String;

    fn is_open(&self) -> bool;

    fn handshake(&self, id: String, hash: SyncHash) -> impl Future<Output = bool>;

    fn sync(&self, state: &SyncMessage) -> impl Future<Output = SyncMessage>;
}
