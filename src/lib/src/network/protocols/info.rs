use iroh::{
    Endpoint, EndpointAddr,
    protocol::{AcceptError, ProtocolHandler},
};

use super::NetworkBuilder;
use crate::network::{RemoteDevice, RemoteDeviceInfo};
use crate::{bitflag, core::config::DsotAppConfig};

static ALPN: &[u8] = b"/dsot/info/v1";

bitflag!(NetworkCapability {
    0 => network "Can connect to the network"
});

impl From<&DsotAppConfig> for NetworkCapability {
    fn from(value: &DsotAppConfig) -> Self {
        let mut v = Self::new();

        if value.value.use_network {
            v.enable_network();
        }

        v
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MachineInfo {
    pub name: String,
    pub desc: String,
    pub cap: NetworkCapability,
}

impl MachineInfo {
    pub fn new(builder: &NetworkBuilder) -> Self {
        let cap = NetworkCapability::from(&*builder.config);
        let (name, desc) = (
            builder.config.value.network_config.public_name.clone(),
            builder.config.value.network_config.public_desc.clone(),
        );

        let name = if let Some(name) = name {
            name
        } else if let Some(hostname) = sysinfo::System::name() {
            hostname
        } else {
            "unknown".to_string()
        };

        let desc = if let Some(desc) = desc {
            desc
        } else if let Some(os) = sysinfo::System::long_os_version() {
            os
        } else {
            "No description".to_string()
        };

        Self { name, desc, cap }
    }
}

#[derive(Debug)]
pub struct InfoProtocol {
    info: MachineInfo,
}

impl InfoProtocol {
    pub fn new(builder: &NetworkBuilder) -> Self {
        Self {
            info: MachineInfo::new(builder),
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
    ) -> crate::error::Result<MachineInfo> {
        let connection = endpoint.connect(target, ALPN).await?;
        let mut reader = crate::network::sink::NetworkReader::open(connection).await?;

        let value = reader.read::<MachineInfo>().await?.ok()?;
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

impl RemoteDeviceInfo for RemoteDevice {
    async fn get_info(&self) -> crate::error::Result<String> {
        let info = InfoProtocol::read(&self.endpoint, self.id).await?;
        Ok(info.name)
    }
}
