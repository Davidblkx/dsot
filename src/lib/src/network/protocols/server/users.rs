use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

use crate::{
    error::Result,
    network::{builder::NetworkBuilder, sink::NetworkChannel},
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
}

impl UsersProtocol {
    pub fn new(builder: &NetworkBuilder) -> Self {
        Self {
            repo: builder.repo.clone(),
        }
    }

    pub async fn reply(&self, connection: Connection) -> Result<()> {
        let mut channel = NetworkChannel::open(connection).await?;

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
