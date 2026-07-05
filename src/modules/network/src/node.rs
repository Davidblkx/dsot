use iroh::{Endpoint, EndpointId};

use crate::{
    DsotNetwork, Result,
    machine_info::MachineInfo,
    protocols::{db_sync::DBSyncProtocol, info::InfoProtocol},
};

#[derive(Debug, Clone)]
pub struct DsotNode {
    pub id: EndpointId,
    pub info: MachineInfo,
    host: Endpoint,
}

impl PartialEq for DsotNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl DsotNetwork {
    pub async fn connect_node(&self, id: EndpointId) -> Result<DsotNode> {
        let info = InfoProtocol::read_info(&self.endpoint, id.clone()).await?;
        Ok(DsotNode {
            id,
            info,
            host: self.endpoint.clone(),
        })
    }
}

impl DsotNode {
    pub async fn sync_database(&self, db: &dsot_db_sync::DsotDatabase) -> Result<()> {
        DBSyncProtocol::sync_database(&self.host, self.id.clone(), db).await?;
        Ok(())
    }
}
