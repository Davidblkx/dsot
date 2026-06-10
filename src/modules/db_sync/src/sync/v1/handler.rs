use crate::{DBSyncError, Result};

use super::model::SyncMessage::{Complete, Error};
use super::model::*;

pub trait SyncBridge {
    fn read_handshake(&mut self) -> impl Future<Output = HandshakeMessage>;
    fn send_handshake(&mut self, msg: &HandshakeMessage) -> impl Future<Output = HandshakeMessage>;
    fn complete_handshake(
        &mut self,
        msg: &HandshakeMessage,
    ) -> impl Future<Output = DataExchangeMessage>;
    fn send_data(&mut self, msg: &DataExchangeMessage)
    -> impl Future<Output = DataExchangeMessage>;
}

pub struct SyncHandler;

macro_rules! eval_send {
    ($i:ident to $rcv:ident) => {
        match &$i {
            Complete => {
                $rcv.send_handshake(&HandshakeMessage::Complete).await;
                return Ok(());
            }
            Error(detail) => {
                let err_message = detail
                    .clone()
                    .unwrap_or("An unknown sync error occur".to_string());
                ::log::error!("Sync error: {0}", err_message);
                $rcv.send_handshake(&HandshakeMessage::Error(Some(err_message.clone())))
                    .await;
                return Err(DBSyncError::SyncError(err_message));
            }
            _ => {}
        };
    };
}

impl SyncHandler {
    /// Establishes a sync connection between two sync bridges.
    /// A handshake is performed first, and then data is exchanged in a loop.
    pub async fn sync<SA: SyncBridge, SB: SyncBridge>(a: &mut SA, b: &mut SB) -> Result<()> {
        let mut msg = a.read_handshake().await;
        eval_send!(msg to b);
        msg = b.send_handshake(&msg).await;
        eval_send!(msg to a);
        let mut data = a.complete_handshake(&msg).await;

        loop {
            eval_send!(data to b);
            data = b.send_data(&data).await;
            eval_send!(data to a);
            data = a.send_data(&data).await;
        }
    }
}
