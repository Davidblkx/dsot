use std::sync::Arc;

use dsot_db_sync::manager::DatabaseManagerProvider;
use dsot_db_sync::sync::SyncHash;
use iroh::endpoint::{Connection, VarInt};
use iroh::protocol::ProtocolHandler;
use iroh::{Endpoint, EndpointId};

use crate::sink::*;

pub const DSOT_DB_SYNC_STATUS_ALPN: &[u8] = b"/dsot/db_sync_status/1";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DatabaseId(String);

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DatabaseSyncHash(dsot_db_sync::sync::SyncHash);

#[derive(Clone)]
pub struct DBSyncStatusProtocol {
    provider: Arc<dyn DatabaseManagerProvider + Send + Sync>,
}

impl std::fmt::Debug for DBSyncStatusProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DBSyncStatusProtocol").finish()
    }
}

impl DBSyncStatusProtocol {
    pub fn new(provider: Arc<dyn DatabaseManagerProvider + Send + Sync>) -> Self {
        Self { provider }
    }

    async fn send_sync_status(&self, connection: Connection) -> crate::Result<()> {
        let mut net = NetworkChannel::open(connection).await?;
        let db_id = match net.read::<DatabaseId>().await? {
            NetworkMessage::Message(db) => db.0,
            NetworkMessage::Disconnect => {
                net.force_close().await;
                return Ok(());
            }
            NetworkMessage::Error(err) => {
                net.force_close().await;
                ::log::debug!("[DBSyncStatusProtocol] Remote error: {0}", err);
                return Ok(());
            }
        };

        let db = self
            .provider
            .provide(db_id.as_str())?
            .open_database()
            .await?;

        let hash = DatabaseSyncHash(db.generate_sync_hash()?);
        net.write(&hash).await?;

        net.close().await
    }

    pub async fn get_sync_status(
        endpoint: &Endpoint,
        id: EndpointId,
        db_id: String,
    ) -> crate::Result<Option<SyncHash>> {
        let db_id = DatabaseId(db_id);

        let connection = endpoint.connect(id, DSOT_DB_SYNC_STATUS_ALPN).await?;
        let mut net = NetworkChannel::start(connection, &Some(db_id)).await?;

        match net.read::<DatabaseSyncHash>().await? {
            NetworkMessage::Message(db) => {
                net.force_close().await;
                Ok(Some(db.0))
            }
            NetworkMessage::Disconnect => {
                ::log::debug!("[DBSyncStatusProtocol] Remote error disconnected");
                net.force_close().await;
                Ok(None)
            }
            NetworkMessage::Error(err) => {
                ::log::debug!("[DBSyncStatusProtocol] Remote error: {0}", err);
                net.force_close().await;
                Ok(None)
            }
        }
    }
}

impl ProtocolHandler for DBSyncStatusProtocol {
    async fn accept(&self, connection: Connection) -> Result<(), iroh::protocol::AcceptError> {
        match self.send_sync_status(connection.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                ::log::debug!("[DBSyncStatusProtocol] Accept error: {0}", e);
                connection.close(VarInt::from_u32(1), b"Internal error");
                Err(iroh::protocol::AcceptError::from_err(e))
            }
        }
    }
}
