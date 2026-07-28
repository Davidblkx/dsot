use iroh::{
    EndpointId,
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

use crate::{
    error::Result,
    network::{
        builder::NetworkBuilder,
        protocols::server::{NetworkValidator, TokenValidator},
    },
    repository::{DeviceRepository, DsotRepository},
    state::devices::RemoteDevice,
};

static ALPN: &[u8] = b"/dsot/users/v1";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum DevicesRequest {
    ListDevices,
    AddDevice(RemoteDevice),
    RemoveDevice(EndpointId),
}

#[derive(Debug)]
pub struct DevicesProtocol {
    repo: DsotRepository,
    validator: TokenValidator,
}

impl DevicesProtocol {
    pub fn new(builder: &NetworkBuilder) -> Self {
        Self {
            repo: builder.repo.clone(),
            validator: builder.get_validator(),
        }
    }

    pub async fn reply(&self, connection: Connection) -> Result<()> {
        let mut channel = self.validator.validate_handshake(connection).await?;

        let req = channel.read::<DevicesRequest>().await?.ok()?;
        match req {
            DevicesRequest::ListDevices => {
                let devices = self.repo.list_devices().await?;
                channel.write(&devices).await?;
            }
            DevicesRequest::AddDevice(device) => {
                self.repo.add_device(device).await?;
                channel.write(&true).await?;
            }
            DevicesRequest::RemoveDevice(id) => {
                self.repo.remove_device(id).await?;
                channel.write(&true).await?;
            }
        }

        channel.close().await?;

        Ok(())
    }
}

impl ProtocolHandler for DevicesProtocol {
    async fn accept(&self, connection: Connection) -> core::result::Result<(), AcceptError> {
        self.reply(connection)
            .await
            .map_err(|err| AcceptError::from_err(err))?;

        Ok(())
    }
}

crate::dsot_protocol!(DevicesProtocol, ALPN);

dsot_remote_protocol!(RemoteDevicesProtocol, devices);

impl<'a> RemoteDevicesProtocol<'a> {
    pub async fn list(&self) -> Result<Vec<RemoteDevice>> {
        exec_request!(self, DevicesRequest::ListDevices)
    }

    pub async fn add(&self, device: RemoteDevice) -> Result<bool> {
        let req = DevicesRequest::AddDevice(device);
        exec_request!(self, req)
    }

    pub async fn remove(&self, id: EndpointId) -> Result<bool> {
        let req = DevicesRequest::RemoveDevice(id);
        exec_request!(self, req)
    }
}
