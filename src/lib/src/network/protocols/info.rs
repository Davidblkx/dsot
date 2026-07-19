use iroh::{
    Endpoint, EndpointAddr,
    protocol::{AcceptError, ProtocolHandler},
};

use super::NetworkBuilder;
use crate::network::{NetworkDevice, NetworkDeviceInfoProvider, device_info::NetworkDeviceInfo};

static ALPN: &[u8] = b"/dsot/info/v1";

#[derive(Debug)]
pub struct InfoProtocol {
    info: NetworkDeviceInfo,
}

impl InfoProtocol {
    pub fn new(builder: &NetworkBuilder) -> Self {
        Self {
            info: NetworkDeviceInfo::new(builder),
        }
    }

    pub async fn send(&self, connection: iroh::endpoint::Connection) -> crate::error::Result<()> {
        let mut sender = crate::network::sink::NetworkWriter::open(connection).await?;

        sender.write(&self.info).await?;

        sender.close().await?;

        Ok(())
    }

    pub async fn read(
        endpoint: &Endpoint,
        target: impl Into<EndpointAddr>,
    ) -> crate::error::Result<NetworkDeviceInfo> {
        let connection = endpoint.connect(target, ALPN).await?;
        let mut reader = crate::network::sink::NetworkReader::open(connection).await?;

        let value = reader.read::<NetworkDeviceInfo>().await?.ok()?;
        reader.close().await;
        Ok(value)
    }
}

impl ProtocolHandler for InfoProtocol {
    async fn accept(
        &self,
        connection: iroh::endpoint::Connection,
    ) -> Result<(), iroh::protocol::AcceptError> {
        self.send(connection)
            .await
            .map_err(|err| AcceptError::from_err(err))?;

        Ok(())
    }
}

crate::dsot_protocol!(InfoProtocol, ALPN);

impl NetworkDeviceInfoProvider for NetworkDevice {
    async fn get_info(&self) -> crate::error::Result<NetworkDeviceInfo> {
        let info = InfoProtocol::read(&self.endpoint, self.id).await?;
        Ok(info)
    }
}
