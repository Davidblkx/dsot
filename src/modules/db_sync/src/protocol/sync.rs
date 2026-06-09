use std::sync::Mutex;

use iroh::endpoint::{Connection, RecvStream, SendStream};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use super::error::SyncResult;
use crate::{
    dser::EntityMessagePack,
    sync::{Handshake, HandshakeResponse, SyncHandler},
};

pub struct RemoteSync {
    pub writer: Mutex<FramedWrite<SendStream, LengthDelimitedCodec>>,
    pub reader: Mutex<FramedRead<RecvStream, LengthDelimitedCodec>>,
}

impl RemoteSync {
    pub async fn connect(conn: Connection) -> Result<Self, iroh::protocol::AcceptError> {
        let (send, receive) = conn.accept_bi().await?;

        let writer = FramedWrite::new(send, LengthDelimitedCodec::new());
        let reader = FramedRead::new(receive, LengthDelimitedCodec::new());

        Ok(Self {
            writer: Mutex::new(writer),
            reader: Mutex::new(reader),
        })
    }

    async fn innser_handshake(&self, req: &Handshake) -> SyncResult<HandshakeResponse> {
        let frame = EntityMessagePack::serialize(req)?;
    }
}

impl SyncHandler for RemoteSync {
    fn name(&self) -> String {
        "Remote Sync".to_string()
    }

    fn is_open(&self) -> bool {
        true
    }

    async fn handshake(&self, req: &Handshake) -> HandshakeResponse {
        match self.innser_handshake(req).await {
            Ok(res) => res,
            Err(err) => {
                log::error!("{}", err);
                HandshakeResponse::error()
            }
        }
    }

    fn sync(
        &self,
        state: &crate::sync::SyncMessage,
    ) -> impl Future<Output = crate::sync::SyncMessage> {
        todo!()
    }
}
