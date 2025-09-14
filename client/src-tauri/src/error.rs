use dsot_runtime::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CommandResult<T> {
    Success { value: T, success: bool },
    Error { message: String, success: bool },
}

impl<T> CommandResult<T> {
    pub fn handle_runtime(result: Result<T>) -> Self {
        match result {
            Ok(value) => CommandResult::Success {
                value,
                success: true,
            },
            Err(err) => CommandResult::Error {
                message: err.to_string(),
                success: false,
            },
        }
    }
}
