//! API to handle node users
//!
//! Contains:
//!   - Implementation for `/dsot/server/users/v1`
//!   - NetworkDevice extension `fn users(&self)`

use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

use super::{NetworkValidator, TokenValidator};
use crate::{
    error::Result,
    network::builder::NetworkBuilder,
    repository::{DsotRepository, UserRepository},
};

static ALPN: &[u8] = b"/dsot/server/users/v1";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum UserRequest {
    LoadUser { user: String, pass: Option<String> },
    ListUsers,
}

#[derive(Debug)]
pub struct UsersProtocol {
    repo: DsotRepository,
    validator: TokenValidator,
}

impl UsersProtocol {
    pub fn new(builder: &NetworkBuilder) -> Self {
        Self {
            repo: builder.repo.clone(),
            validator: builder.get_validator(),
        }
    }

    pub async fn reply(&self, connection: Connection) -> Result<()> {
        let mut channel = self.validator.validate_handshake(connection).await?;

        let req = channel.read::<UserRequest>().await?.ok()?;
        match req {
            UserRequest::LoadUser { user, pass } => {
                let user = self.repo.load_user(user.as_str(), pass).await?;
                channel.write(&user).await?;
            }
            UserRequest::ListUsers => {
                let users = self.repo.list_users().await?;
                channel.write(&users).await?;
            }
        };

        channel.close().await?;

        Ok(())
    }
}

impl ProtocolHandler for UsersProtocol {
    async fn accept(&self, connection: Connection) -> core::result::Result<(), AcceptError> {
        self.reply(connection)
            .await
            .map_err(|err| AcceptError::from_err(err))?;

        Ok(())
    }
}

crate::dsot_protocol!(UsersProtocol, ALPN);

impl_network_device_extension!(RemoteUsersProtocol, users);

impl<'a> RemoteUsersProtocol<'a> {
    pub async fn load(&self, user: &str, pass: Option<String>) -> Result<String> {
        let req = UserRequest::LoadUser {
            user: user.to_string(),
            pass,
        };

        exec_request!(self, req)
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        exec_request!(self, UserRequest::ListUsers)
    }
}
