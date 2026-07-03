use futures_util::{SinkExt, StreamExt};
use iroh::{
    Endpoint, EndpointId,
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};
use tokio_util::{
    bytes::Bytes,
    codec::{FramedRead, FramedWrite, LengthDelimitedCodec},
};

use crate::{DsotNetworkError, NetworkInitOptions, Result, machine_info::MachineInfo};

pub const DSOT_INFO_ALPN_V1: &[u8] = b"/dsot/info/1";

#[derive(Debug, Clone)]
pub struct InfoProtocol {
    pub info: MachineInfo,
}

impl InfoProtocol {
    pub fn new(options: &NetworkInitOptions) -> Self {
        Self {
            info: MachineInfo::new(options),
        }
    }

    pub async fn read_info(endpoint: &Endpoint, id: EndpointId) -> Result<MachineInfo> {
        ::log::debug!("Reading info from endpoint: {}", id);
        let conn = endpoint.connect(id, DSOT_INFO_ALPN_V1).await?;
        let stream_reader = conn
            .accept_uni()
            .await
            .map_err(|e| AcceptError::from_err(e))?;
        let mut reader = FramedRead::new(stream_reader, LengthDelimitedCodec::new());

        if let Some(bytes) = reader.next().await {
            ::log::debug!("Read info: {:?}", bytes);
            Ok(MachineInfo::from_binary(bytes?.iter().as_slice())?)
        } else {
            ::log::warn!("No info received from endpoint: {}", id);
            Err(DsotNetworkError::EmptyMessage)
        }
    }
}

impl ProtocolHandler for InfoProtocol {
    async fn accept(&self, connection: Connection) -> std::result::Result<(), AcceptError> {
        ::log::debug!("Accepting info from connection");
        let stream_writer = connection.open_uni().await?;
        let mut writer = FramedWrite::new(stream_writer, LengthDelimitedCodec::new());

        let bytes = match self.info.to_binary() {
            Ok(bytes) => Bytes::from(bytes),
            Err(e) => return Err(AcceptError::from_err(e)),
        };

        ::log::debug!("Sending info: {:?}", bytes);
        writer.send(bytes).await?;
        writer.into_inner().finish()?;
        ::log::debug!("Info sent");

        let _ = connection.closed().await;

        Ok(())
    }
}

pub trait RegisterInfoProtocol {
    fn register_info_protocol(self, options: &NetworkInitOptions) -> Self;
}

impl RegisterInfoProtocol for iroh::protocol::RouterBuilder {
    fn register_info_protocol(self, options: &NetworkInitOptions) -> Self {
        self.accept(DSOT_INFO_ALPN_V1, InfoProtocol::new(options))
    }
}
