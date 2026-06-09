use crate::{
    DsotDatabase,
    database::{DsotDatabaseTransaction, Result},
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum SyncStatus {
    HandshakeRequest {
        id: String,
        hash: [u8; 32],
    },
    HandshakeResponse {
        need_sync: bool,
    },
    Exchange {
        journal_keys: Vec<[u8; 16]>,
        missing_keys: Vec<[u8; 16]>,
        entries: Vec<Vec<u8>>,
    },
    Complete,
    Fail(String),
}

impl DsotDatabase {
    pub async fn handshake_sync(
        &self,
        id: String,
        hash: [u8; 32],
    ) -> Result<(bool, Option<DsotDatabaseTransaction<'_>>)> {
        if &self.id != &id || hash == self.generate_sync_hash()? {
            return Ok((false, None));
        }

        let trx = self.begin_transaction().await?;

        Ok((true, Some(trx)))
    }
}
