use iroh::{Endpoint, EndpointId};

use crate::{error::Result, network::DsotNetwork};

#[derive(Debug)]
pub struct RemoteDevice {
    pub(crate) endpoint: Endpoint,
    pub id: EndpointId,
}

impl DsotNetwork {
    pub async fn connect_remote_device(&self, id: EndpointId) -> Result<RemoteDevice> {
        let endpoint = self.connect().await?;
        Ok(RemoteDevice { endpoint, id })
    }
}

pub trait RemoteDeviceInfo {
    fn get_info(&self) -> impl Future<Output = Result<String>>;
}
