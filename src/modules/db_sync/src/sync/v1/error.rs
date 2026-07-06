use serde::{Deserialize, Serialize};

use crate::error::DBSyncError;

use super::model::DBSyncMessage;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum SyncError {
    Unknwon,
    Deserialize,
    Serialize,
    MissingHandshake,
    DatabaseMissmatch,
}

impl SyncError {
    pub fn to_msg_string(&self) -> &'static str {
        match self {
            SyncError::Unknwon => "Unknown error",
            SyncError::Deserialize => "Failed to deserialize remote message",
            SyncError::Serialize => "Failed to serialize local message",
            SyncError::MissingHandshake => "Missing handshake: expected 'Hello' message",
            SyncError::DatabaseMissmatch => "Database id mismatch",
        }
    }

    pub fn to_error(&self) -> DBSyncError {
        DBSyncError::SyncError(self.to_msg_string().to_string())
    }

    pub fn to_sync_message(self) -> DBSyncMessage {
        DBSyncMessage::Error(self)
    }
}
