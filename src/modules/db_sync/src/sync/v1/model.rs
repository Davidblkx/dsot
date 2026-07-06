use dsot_serde::BinarySerde;
use serde::{Deserialize, Serialize};

use super::error::SyncError;
use crate::Result;

pub type SyncHash = [u8; 32];
pub type SyncKey = [u8; 16];
pub type SyncEntry = Vec<u8>;

#[derive(Debug, Deserialize, Serialize)]
pub enum DBSyncMessage {
    // Always sent by the node starting a sync session.
    Hello(String),
    // Used to validate hash of two nodes' databases.
    Validate(SyncHash),
    // All the keys that this node contains.
    BeginExchange(Vec<SyncKey>),
    // Sent by the node to exchange missing entries and keys.
    Exchange {
        // All the keys that this node contains.
        keys: Vec<SyncKey>,
        // Keys this node is missing.
        request: Vec<SyncKey>,
        // The entries requested in this exchange.
        entries: Vec<SyncEntry>,
    },
    // Sent when the node has finished exchanging keys and entries.
    Completed,
    // Sent when the node encounters an error during sync.
    Error(SyncError),
}

impl DBSyncMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        match BinarySerde::serialize(&self) {
            Ok(bytes) => Ok(bytes),
            Err(e) => {
                ::log::error!("Failed to serialize sync message: {}", e);
                let err_msg = DBSyncMessage::Error(SyncError::Serialize);
                Ok(BinarySerde::serialize(&err_msg)?)
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        match BinarySerde::deserialize(bytes) {
            Ok(message) => message,
            Err(e) => {
                ::log::error!("Failed to deserialize sync message: {}", e);
                DBSyncMessage::Error(SyncError::Deserialize)
            }
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, DBSyncMessage::Error(_))
    }
}
