use iroh::EndpointId;

use crate::{DsotNetwork, Result, machine_info::MachineInfo, protocols::info::InfoProtocol};

#[derive(Debug, Clone, PartialEq)]
pub struct DsotNode {
    pub id: EndpointId,
    pub info: MachineInfo,
}

impl DsotNetwork {
    pub async fn connect_node(&self, id: EndpointId) -> Result<DsotNode> {
        let info = InfoProtocol::read_info(&self.endpoint, id.clone()).await?;
        Ok(DsotNode { id, info })
    }
}
