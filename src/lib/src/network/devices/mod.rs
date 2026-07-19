pub mod device_info;

use iroh::{Endpoint, EndpointId};

use crate::{error::Result, network::DsotNetwork};

#[derive(Debug, Clone)]
pub struct NetworkDevice {
    pub(crate) endpoint: Endpoint,
    pub id: EndpointId,
}

impl DsotNetwork {
    pub async fn connect_remote_device(&self, id: EndpointId) -> Result<NetworkDevice> {
        let endpoint = self.connect().await?;
        Ok(NetworkDevice { endpoint, id })
    }
}

pub trait NetworkDeviceInfoProvider {
    fn get_info(&self) -> impl Future<Output = Result<device_info::NetworkDeviceInfo>>;
}
