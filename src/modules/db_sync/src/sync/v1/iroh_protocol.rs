use crate::{DatabaseManager, sync::SyncBridge};

use super::model::*;
use super::{
    db_sync_bridge::DatabaseSyncBridge, handler::SyncHandler, iroh_sync_bridge::IrohSyncBridge,
};
use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

pub const DSOT_DB_SYNC_ALPN_V1: &[u8] = b"/dsot/db_sync/1";

#[derive(Clone, Debug)]
pub struct DBSyncProtocol {
    get_manager: fn(id: &str) -> crate::Result<DatabaseManager>,
}

impl ProtocolHandler for DBSyncProtocol {
    async fn accept(&self, connection: Connection) -> Result<(), iroh::protocol::AcceptError> {
        let mut remote_bridge = IrohSyncBridge::create(connection).await?;

        let db = match (self.get_manager)(remote_bridge.id.as_str()) {
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
