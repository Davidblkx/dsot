pub type SyncHash = [u8; 32];
pub type SyncKey = [u8; 16];

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SyncMessage {
    Start(Vec<SyncKey>),
    Exchange {
        request_entries: Vec<SyncKey>,
        available_keys: Vec<SyncKey>,
        requested_entries: Vec<Vec<u8>>,
    },
    Complete,
    Fail(String),
}

impl SyncMessage {
    pub fn new_fail<T: ToString>(message: T) -> Self {
        Self::Fail(message.to_string())
    }
}

impl ToString for SyncMessage {
    fn to_string(&self) -> String {
        match self {
            Self::Exchange {
                available_keys,
                request_entries,
                requested_entries,
            } => format!(
                "Exchange {{ available_keys: {}, request_entries: {}, requested_entries: {} }}",
                available_keys.len(),
                request_entries.len(),
                requested_entries.len()
            ),
            Self::Complete => "Complete".to_string(),
            Self::Fail(message) => format!("Fail({})", message.clone()),
            Self::Start(keys) => format!("Start({})", keys.len()),
        }
    }
}
