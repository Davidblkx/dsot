use dsot_db_sync::{
    DatabaseManager, DsotDatabase,
    sync::{DatabaseSyncNode, SyncNode, SyncNodeHandler},
};
use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

use crate::{
    error::Result,
    network::{NetworkDevice, builder::NetworkBuilder},
    repository::{DsotRepository, UserRepository},
};

static ALPN: &[u8] = b"/dsot/info/v1";

#[derive(Debug)]
pub struct DBSyncProtocol {
    repo: DsotRepository,
}

impl DBSyncProtocol {
    pub fn new(builder: &NetworkBuilder) -> Self {
        Self {
            repo: builder.repo.clone(),
        }
    }

    async fn handle_sync(&self, connection: Connection) -> Result<()> {
        let mut net_bridge = super::sync_node::NetworkSyncNode::await_sync(connection).await?;
        let id = net_bridge
            .get_db_id()
            .await
            .ok_or(dsot_db_sync::DBSyncError::SyncError(
                "Database id to sync not defined".to_string(),
            ))?;

        // TODO: Use safe method that doesn't require pass
        let user_path = self.repo.load_user(id.as_str(), None).await?;
        let manager = DatabaseManager::open_folder(user_path)?;
        let db = manager.open_database().await?;
        let mut local_bridge = DatabaseSyncNode::create(&db).await?;

        SyncNodeHandler::sync(&mut net_bridge, &mut local_bridge).await?;

        net_bridge.channel.force_close().await;
        local_bridge.close().await?;

        Ok(())
    }
}

impl ProtocolHandler for DBSyncProtocol {
    async fn accept(
        &self,
        connection: iroh::endpoint::Connection,
    ) -> std::result::Result<(), iroh::protocol::AcceptError> {
        self.handle_sync(connection)
            .await
            .map_err(|err| AcceptError::from_err(err))?;

        Ok(())
    }
}

crate::dsot_protocol!(DBSyncProtocol, ALPN);

impl NetworkDevice {
    pub async fn sync_database(&self, db: &DsotDatabase) -> Result<()> {
        let db_id = db.get_id().to_string();
        let mut local_bridge = DatabaseSyncNode::create(db).await?;

        let connection = self.connect_alpn(ALPN).await?;
        let mut net_bridge =
            super::sync_node::NetworkSyncNode::start_sync(connection, db_id).await?;

        SyncNodeHandler::sync(&mut local_bridge, &mut net_bridge).await?;

        let local_close = local_bridge.close().await;
        let net_close = net_bridge.channel.close().await;

        local_close?;
        net_close?;

        Ok(())
    }
}
