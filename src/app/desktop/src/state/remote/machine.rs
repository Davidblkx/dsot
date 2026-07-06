use dsot_network::{DsotNode, NetworkAddress};

#[derive(Debug, Clone, PartialEq)]
pub enum MachineStatus {
    Offline,
    Online(DsotNode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Disabled,
    Pending,
    InSync,
    Failure,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NewNetworkAddress {
    pub id: String,
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteMachine {
    pub id: iroh::EndpointId,
    pub name: String,
    pub desc: String,
    pub status: MachineStatus,
    pub sync: SyncStatus,
}

impl From<NetworkAddress> for RemoteMachine {
    fn from(value: NetworkAddress) -> Self {
        Self {
            id: value.address,
            name: value.name,
            desc: value.desc,
            status: MachineStatus::Offline,
            sync: SyncStatus::Disabled,
        }
    }
}

impl Into<NetworkAddress> for RemoteMachine {
    fn into(self) -> NetworkAddress {
        NetworkAddress {
            address: self.id,
            name: self.name,
            desc: self.desc,
        }
    }
}
