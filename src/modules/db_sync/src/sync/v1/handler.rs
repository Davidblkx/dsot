use crate::error::*;

use super::model::DBSyncMessage;

/// A trait for a node capable of syncing with a database.
pub trait SyncNode {
    /// Return the id of the current database being sync, None if no db is open to sync
    fn get_db_id(&mut self) -> impl Future<Output = Option<String>>;
    /// Handle a sync message from the peer, returning a response message.
    fn handle(&mut self, message: &DBSyncMessage) -> impl Future<Output = Result<DBSyncMessage>>;
}

pub struct SyncNodeHandler;

macro_rules! send {
    ($msg: ident to $target: ident) => {
        match $msg {
            DBSyncMessage::Completed => {
                let _ = $target.handle(&$msg).await;
                return Ok(());
            }
            DBSyncMessage::Error(err) => {
                let _ = $target.handle(&DBSyncMessage::Error(err.clone())).await;
                return Err(err.to_error());
            }
            e => $target.handle(&e).await?,
        }
    };
}

impl SyncNodeHandler {
    pub async fn sync<NodeA: SyncNode, NodeB: SyncNode>(
        a: &mut NodeA,
        b: &mut NodeB,
    ) -> Result<()> {
        let db_id = match a.get_db_id().await {
            Some(id) => id,
            None => return Err(DBSyncError::NoOpenConnection),
        };

        let mut msg = DBSyncMessage::Hello(db_id);

        loop {
            msg = send!(msg to b);
            msg = send!(msg to a);
        }
    }
}
