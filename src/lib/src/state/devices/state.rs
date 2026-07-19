#[derive(Debug, Clone)]
pub struct RemoteDevice {
    pub id: iroh::EndpointId,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct RemoteDevices {
    pub devices: Vec<RemoteDevice>,
}
