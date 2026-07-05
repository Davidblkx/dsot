use serde::{Deserialize, Serialize};

use crate::error::DBSyncError;

#[derive(Debug, Deserialize, Serialize)]
pub enum SyncError {
    Unknwon,
    Deserialize,
    Serialize,
    MissingHandshake,
}

impl SyncError {
    pub fn to_error_message(&self) -> &'static str {
        match self {
            SyncError::Unknwon => "Unknown error",
            SyncError::Deserialize => "Failed to deserialize remote message",
            SyncError::Serialize => "Failed to serialize local message",
            SyncError::MissingHandshake => "Missing handshake: expected 'Hello' message",
        }
    }

    pub fn to_error(&self) -> DBSyncError {
        DBSyncError::SyncError(self.to_error_message().to_string())
    }
}
