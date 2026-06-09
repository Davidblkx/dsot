use super::model::{Handshake, HandshakeResponse, SyncMessage};

pub trait SyncHandler {
    fn name(&self) -> String;

    fn is_open(&self) -> bool;

    fn handshake(&self, value: &Handshake) -> impl Future<Output = HandshakeResponse>;

    fn sync(&self, state: &SyncMessage) -> impl Future<Output = SyncMessage>;
}
