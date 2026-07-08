use dsot_network::{DsotNode, NetworkAddress};

#[derive(Debug, Clone, PartialEq)]
pub enum RemoteNodeStatus {
    Offline,
    Online(DsotNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteNode {
    pub id: iroh::EndpointId,
    pub status: RemoteNodeStatus,
    pub address: NetworkAddress,
}

impl From<NetworkAddress> for RemoteNode {
    fn from(value: NetworkAddress) -> Self {
        Self {
            id: value.address,
            address: value,
            status: RemoteNodeStatus::Offline,
        }
    }
}

impl Into<NetworkAddress> for RemoteNode {
    fn into(self) -> NetworkAddress {
        self.address
    }
}

impl Default for RemoteNode {
    fn default() -> Self {
        let id = iroh::SecretKey::generate().public();

        Self {
            id: id.clone(),
            status: RemoteNodeStatus::Offline,
            address: NetworkAddress {
                address: id,
                desc: "Invalid selected index".to_string(),
                name: "Empty".to_string(),
            },
        }
    }
}
