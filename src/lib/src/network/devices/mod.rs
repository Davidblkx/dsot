//! Network device and related utilities.
//!
//! Module contains business logic to communicate and interact with network devices.

pub mod device_info;

use iroh::{Endpoint, EndpointId, endpoint::Connection};

use crate::{error::Result, network::DsotNetwork};

/// Represents a network device connected to the Dsot network.
///
/// This is the gateway to interacting with a specific network device.
#[derive(Debug, Clone)]
pub struct NetworkDevice {
    pub(crate) endpoint: Endpoint,
    pub id: EndpointId,
}

impl NetworkDevice {
    /// Connects to the device using the given ALPN protocol.
    pub async fn connect_alpn(&self, alpn: &[u8]) -> Result<Connection> {
        let connection = self.endpoint.connect(self.id, alpn).await?;
        Ok(connection)
    }
}

impl DsotNetwork {
    /// Starts a connection to an endpoint
    ///
    /// This guarantees the network is initialized but does not send any request to device
    pub async fn connect_remote_device(&self, id: EndpointId) -> Result<NetworkDevice> {
        let endpoint = self.connect().await?;
        Ok(NetworkDevice { endpoint, id })
    }
}

pub trait NetworkDeviceInfoProvider {
    fn get_info(&self) -> impl Future<Output = Result<device_info::NetworkDeviceInfo>>;
}
