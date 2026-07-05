use crate::error::*;

use super::model::DBSyncMessage;

/// A trait for a node capable of syncing with a database.
pub trait SyncNode {
    /// Return the id of the current database being sync, None if no db is open to sync
    fn get_db_id(&mut self) -> impl Future<Output = Option<String>>;
    /// Handle a sync message from the peer, returning a response message.
    fn handle(&mut self, message: &DBSyncMessage) -> impl Future<Output = DBSyncMessage>;
}

pub struct SyncNodeHandler;

impl SyncNodeHandler {
    pub async fn sync<NodeA: SyncNode, NodeB: SyncNode>(a: NodeA, b: NodeB) -> Result<()> {
        todo!()
    }
}
