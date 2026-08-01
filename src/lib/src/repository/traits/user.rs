use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait UserRepository: Send + Sync + std::fmt::Debug {
    /// Load the list of available users
    async fn list_users(&self) -> Result<Vec<String>>;
}
