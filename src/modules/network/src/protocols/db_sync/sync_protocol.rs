use std::sync::Arc;

use dsot_db_sync::{
    DsotDatabase,
    manager::DatabaseManagerProvider,
    sync::{DatabaseSyncBridge, SyncHandler},
};
use iroh::{Endpoint, EndpointId, endpoint::Connection, protocol::ProtocolHandler};

use crate::{NetworkInitOptions, protocols::db_sync::bridge::NetworkSyncBridge};

pub const DSOT_DB_SYNC_ALPN_V1: &[u8] = b"/dsot/db_sync/1";

#[derive(Clone)]
pub struct DBSyncProtocol {
    provider: Arc<dyn DatabaseManagerProvider + Send + Sync>,
}

impl std::fmt::Debug for DBSyncProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DBSyncProtocol").finish()
    }
}

impl DBSyncProtocol {
    pub fn new(provider: Arc<dyn DatabaseManagerProvider + Send + Sync>) -> Self {
        Self { provider }
    }

    async fn sync(&self, connection: Connection) -> crate::error::Result<()> {
        let mut remote_bridge = NetworkSyncBridge::start_passive_sync(connection).await?;
        let id = remote_bridge.read_hello_handshake().await?;
        let db = self.provider.provide(&id)?.open_database().await?;
        let mut local_bridge = DatabaseSyncBridge::create(&db).await?;

        SyncHandler::sync(&mut remote_bridge, &mut local_bridge).await?;

        remote_bridge.channel.force_close().await;

        Ok(())
    }

    pub async fn sync_database(
        endpoint: &Endpoint,
        id: EndpointId,
        db: &DsotDatabase,
    ) -> crate::error::Result<()> {
        let mut local_bridge = DatabaseSyncBridge::create(&db).await?;

        let conn = endpoint.connect(id, DSOT_DB_SYNC_ALPN_V1).await?;
        let mut remote_bridge =
            NetworkSyncBridge::start_sync(conn, local_bridge.get_hello_message()).await?;

        SyncHandler::sync(&mut local_bridge, &mut remote_bridge).await?;

        remote_bridge.channel.close().await?;

        Ok(())
    }
}

impl ProtocolHandler for DBSyncProtocol {
    async fn accept(
        &self,
        connection: iroh::endpoint::Connection,
    ) -> Result<(), iroh::protocol::AcceptError> {
        match self.sync(connection).await {
            Ok(_) => Ok(()),
            Err(e) => Err(iroh::protocol::AcceptError::from_err(e)),
        }
    }
}

pub trait RegisterSyncProtocolV1 {
    fn register_sync_protocol_v1(self, options: &NetworkInitOptions) -> Self;
}

impl RegisterSyncProtocolV1 for iroh::protocol::RouterBuilder {
    fn register_sync_protocol_v1(self, options: &NetworkInitOptions) -> Self {
        if options.config.use_db_sync {
            self.accept(
                DSOT_DB_SYNC_ALPN_V1,
                DBSyncProtocol::new(options.manager.clone()),
            )
        } else {
            self
        }
    }
}
