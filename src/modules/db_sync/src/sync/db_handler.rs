use std::cell::RefCell;

use super::SyncHandler;
use crate::{
    DsotDatabase,
    database::{DsotDatabaseTransaction, Result},
    sync::{Handshake, HandshakeResponse, SyncMessage},
};

pub struct DsotDatabaseSyncHandler<'a> {
    trx: RefCell<DsotDatabaseTransaction<'a>>,
    db: &'a DsotDatabase,
}

impl DsotDatabase {
    pub async fn create_sync_handler<'a>(&'a self) -> Result<DsotDatabaseSyncHandler<'a>> {
        let trx = self.begin_transaction().await?;
        let handler = DsotDatabaseSyncHandler {
            trx: RefCell::new(trx),
            db: self,
        };

        Ok(handler)
    }
}

impl<'a> DsotDatabaseSyncHandler<'a> {
    pub async fn commit(self) -> Result<()> {
        let trx = self.trx.into_inner();
        trx.commit().await?;
        Ok(())
    }
}

impl<'a> SyncHandler for DsotDatabaseSyncHandler<'a> {
    fn name(&self) -> String {
        self.db.id.clone()
    }

    fn is_open(&self) -> bool {
        true
    }

    async fn handshake(&self, req: &Handshake) -> HandshakeResponse {
        if req.id != self.db.id {
            return HandshakeResponse::fail_match();
        }

        match self.trx.borrow().generate_sync_hash() {
            Ok(current_hash) => HandshakeResponse::need(current_hash != req.hash),
            Err(_) => HandshakeResponse::error(),
        }
    }

    async fn sync(&self, state: &SyncMessage) -> SyncMessage {
        let mut trx = self.trx.borrow_mut();
        match trx.remote_sync(state).await {
            Err(err) => SyncMessage::Fail(err.to_string()),
            Ok(response) => response,
        }
    }
}
