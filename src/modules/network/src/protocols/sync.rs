use std::sync::Arc;

use dsot_db_sync::{manager::DatabaseManagerProvider, sync::SyncBridge};

use dsot_db_sync::sync::model::*;
use dsot_db_sync::sync::{
    db_sync_bridge::DatabaseSyncBridge, handler::SyncHandler, iroh_sync_bridge::IrohSyncBridge,
};
use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

use crate::NetworkInitOptions;

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
}

impl ProtocolHandler for DBSyncProtocol {
    async fn accept(&self, connection: Connection) -> Result<(), iroh::protocol::AcceptError> {
        let mut remote_bridge = IrohSyncBridge::create_active(connection).await?;

        let db = match self.provider.provide(remote_bridge.id.as_str()) {
            Ok(manager) => match manager.open_database().await {
                Ok(db) => db,
                Err(e) => {
                    remote_bridge
                        .send_handshake(&HandshakeMessage::error(e.to_string()))
                        .await;
                    return Err(AcceptError::from_err(e));
                }
            },
            Err(e) => {
                remote_bridge
                    .send_handshake(&HandshakeMessage::error(e.to_string()))
                    .await;
                return Err(AcceptError::from_err(e));
            }
        };

        let mut local_bridge = match DatabaseSyncBridge::create(&db).await {
            Ok(bridge) => bridge,
            Err(e) => {
                remote_bridge
                    .send_handshake(&HandshakeMessage::error(e.to_string()))
                    .await;
                return Err(AcceptError::from_err(e));
            }
        };

        match SyncHandler::sync(&mut remote_bridge, &mut local_bridge).await {
            Ok(_) => {
                local_bridge
                    .close()
                    .await
                    .map_err(|e| AcceptError::from_err(e))?;
                Ok(())
            }
            Err(e) => {
                local_bridge
                    .close()
                    .await
                    .map_err(|e| AcceptError::from_err(e))?;
                Err(AcceptError::from_err(e))
            }
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
