pub type SyncHash = [u8; 32];
pub type SyncKey = [u8; 16];

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Handshake {
    pub id: String,
    pub hash: SyncHash,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct HandshakeResponse {
    pub id_match: bool,
    pub need_sync: bool,
    pub error: bool,
}

impl HandshakeResponse {
    pub fn fail_match() -> Self {
        Self {
            id_match: false,
            need_sync: false,
            error: false,
        }
    }

    pub fn error() -> Self {
        Self {
            id_match: true,
            need_sync: false,
            error: true,
        }
    }

    pub fn need(need_sync: bool) -> Self {
        Self {
            id_match: true,
            need_sync,
            error: false,
        }
    }
}

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
