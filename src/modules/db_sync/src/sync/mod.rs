mod execute;
mod handler;
mod model;

use crate::RepositoryRegistry;

use super::{DsotDatabase, database::Result};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SyncHandshakeRequest {
    pub id: String,
    pub sync: [u8; 32],
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SyncHandshakeResponse {
    pub need_sync: bool,
    pub id_match: bool,
}

impl SyncHandshakeResponse {
    pub fn wrong_id() -> Self {
        Self {
            need_sync: false,
            id_match: false,
        }
    }

    pub fn need_sync(need_sync: bool) -> Self {
        Self {
            need_sync,
            id_match: true,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SyncStartRequest {
    pub keys: Vec<[u8; 16]>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SyncExchange {
    pub keys: Vec<[u8; 16]>,
    pub entries: Vec<Vec<u8>>,
}

impl SyncExchange {
    pub fn empty() -> Self {
        Self {
            keys: Vec::new(),
            entries: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty() && self.entries.is_empty()
    }
}

impl DsotDatabase {
    pub fn sync_handshake(&self, req: SyncHandshakeRequest) -> Result<SyncHandshakeResponse> {
        if self.id != req.id {
            return Ok(SyncHandshakeResponse::wrong_id());
        }

        let need_sync = req.sync != self.generate_sync_hash()?;

        Ok(SyncHandshakeResponse::need_sync(need_sync))
    }

    pub fn start_sync(&self, req: &SyncStartRequest) -> Result<SyncExchange> {
        if req.keys.is_empty() {
            log::debug!("Sync: empty keys, skipping");
            return Ok(SyncExchange::empty());
        }

        let missing_keys = self.get_keys_not_in_journal(&req.keys)?;
        log::info!("Sync: detected {} missing keys", missing_keys.len());
        let missing_entries = self.get_journal_entries_not_in_array(&req.keys)?;
        log::info!("Sync: detected {} missing entries", missing_entries.len());

        Ok(SyncExchange {
            entries: missing_entries,
            keys: missing_keys,
        })
    }

    pub async fn sync(&self, ex: &SyncExchange) -> Result<SyncExchange> {
        if ex.is_empty() {
            log::debug!("Sync: empty exchange, skipping");
            return Ok(SyncExchange::empty());
        }

        let missing_entries = match &ex.keys.is_empty() {
            false => {
                let keys = self.get_journal_entries_in_array(&ex.keys)?;
                log::info!("Sync: detected {} missing entries on remote", keys.len());
                keys
            }
            _ => Vec::new(),
        };

        let entries: Vec<&[u8]> = ex.entries.iter().map(|v| v.as_slice()).collect();
        log::info!("Sync: Aplying {} entries", &ex.entries.len());
        let ids = RepositoryRegistry::instance()
            .apply_journals_bytes(self, &entries)
            .await?;
        log::info!("Sync: Applied {} entries", ids.len());

        Ok(SyncExchange {
            keys: Vec::new(),
            entries: missing_entries,
        })
    }
}
