use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

use super::{NetworkValidator, TokenValidator};
use crate::{
    error::Result,
    network::{NetworkDevice, builder::NetworkBuilder, sink::NetworkChannel},
    repository::{DsotRepository, UserRepository},
};

static ALPN: &[u8] = b"/dsot/users/v1";

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

pub struct RemoteUsersProtocol<'a> {
    device: &'a NetworkDevice,
    validator: TokenValidator,
}

impl NetworkDevice {
    pub fn users(&self, validator: impl NetworkValidator) -> RemoteUsersProtocol<'_> {
        RemoteUsersProtocol {
            device: self,
            validator: validator.get_validator(),
        }
    }
}

impl<'a> RemoteUsersProtocol<'a> {
    async fn connect(&self) -> Result<NetworkChannel> {
        let connection = self.device.connect_alpn(ALPN).await?;
        self.validator.start_handshake(connection).await
    }

    pub async fn load_user(&self, user: &str, pass: Option<String>) -> Result<String> {
        let mut channel = self.connect().await?;

        let result = channel
            .request(UserRequest::LoadUser {
                user: user.to_string(),
                pass,
            })
            .await;

        channel.force_close().await;

        result
    }

    pub async fn list_users(&self) -> Result<Vec<String>> {
        let mut channel = self.connect().await?;

        let result = channel.request(UserRequest::ListUsers).await;

        channel.force_close().await;

        result
    }
}
