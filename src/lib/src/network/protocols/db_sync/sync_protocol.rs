use std::path::PathBuf;

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
};

static ALPN: &[u8] = b"/dsot/db_sync/v1";

/// Iroh protocol `/dsot/db_sync/v1` used to sync two nodes's database
#[derive(Debug)]
pub struct DBSyncProtocol {
    root_path: PathBuf,
}

impl DBSyncProtocol {
    pub fn new(builder: &NetworkBuilder) -> Self {
        Self {
            root_path: builder.config.data_dir.clone(),
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

        let user_path = self.root_path.join(id);
        if !user_path.exists() {
            ::log::debug!("User path does not exist: {:?}", user_path);
            net_bridge
                .channel
                .write_error("User path does not exist")
                .await?;
            return Ok(());
        }

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
    /// Sync a database with a network device
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
