use dsot_network::DsotNode;

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteDevice {
    id: iroh::EndpointId,
    name: String,
    description: String,
    status: RemoteDeviceStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RemoteDeviceStatus {
    Offline,
    Online(DsotNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoteDevices {
    pub devices: Vec<RemoteDevice>,
}
