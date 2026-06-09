use iroh::protocol::ProtocolHandler;

use crate::DatabaseManager;

pub type SyncDatabaseError = crate::DBSyncError;
pub type SyncResult<T> = std::result::Result<T, crate::DBSyncError>;
mod sync;

pub const DB_SYNC_PROTOCOL_ALPN: &[u8] = b"/dsot/db_sync/1";

#[derive(Debug, Clone)]
pub struct DbSyncProtocol {
    provider: fn(&str) -> crate::manager::Result<DatabaseManager>,
}

impl ProtocolHandler for DbSyncProtocol {
    fn accept(
        &self,
        connection: iroh::endpoint::Connection,
    ) -> impl Future<Output = Result<(), iroh::protocol::AcceptError>> + Send {
        Box::pin(async move {
            let connection = connection.accept_bi().await?;

            Ok(())
        })
    }
}
