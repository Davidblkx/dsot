use std::sync::OnceLock;

use iroh::endpoint::Connection;

use crate::{
    core::config::DsotAppConfig,
    error::{DsotError, Result},
    network::{builder::NetworkBuilder, sink::NetworkChannel},
};

static TOKEN: OnceLock<String> = OnceLock::new();

fn get_token() -> Result<&'static str> {
    match TOKEN.get() {
        Some(token) => Ok(token),
        None => Err(DsotError::InvalidToken),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TokenValidator(());

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum TokenRequest {
    Validate(String),
    ValidateSuccess,
    ValidateFail,
}

impl TokenValidator {
    pub fn new(token: impl ToString) -> Self {
        let _ = TOKEN.get_or_init(|| token.to_string());

        TokenValidator(())
    }

    pub async fn start_handshake(&self, connection: Connection) -> Result<NetworkChannel> {
        let token = get_token()?;
        let mut channel = NetworkChannel::open(connection).await?;
        channel
            .write(&TokenRequest::Validate(token.to_string()))
            .await?;
        let result = channel.read::<TokenRequest>().await?.ok()?;
        match result {
            TokenRequest::ValidateSuccess => Ok(channel),
            _ => Err(DsotError::InvalidToken),
        }
    }

    pub async fn validate_handshake(&self, connection: Connection) -> Result<NetworkChannel> {
        let token = get_token()?;
        let mut channel = NetworkChannel::open(connection).await?;

        let handshake = channel.read::<TokenRequest>().await?.ok()?;
        let success = match handshake {
            TokenRequest::Validate(remote_token) => token == remote_token.as_str(),
            _ => false,
        };

        if success {
            channel.write(&TokenRequest::ValidateSuccess).await?;

            Ok(channel)
        } else {
            channel.write(&TokenRequest::ValidateFail).await?;
            channel.close().await?;

            Err(DsotError::InvalidToken)
        }
    }
}

pub trait NetworkValidator {
    fn get_validator(&self) -> TokenValidator;
}

impl NetworkValidator for DsotAppConfig {
    fn get_validator(&self) -> TokenValidator {
        TokenValidator::new(self.value.token.as_str())
    }
}

impl NetworkValidator for NetworkBuilder {
    fn get_validator(&self) -> TokenValidator {
        self.config.get_validator()
    }
}
