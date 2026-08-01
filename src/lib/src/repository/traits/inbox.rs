use async_trait::async_trait;

use crate::{
    error::Result,
    state::inbox::{InboxFilter, InboxItemValue},
};

#[async_trait]
pub trait InboxRepository {
    async fn load_inbox(&self, filter: &InboxFilter) -> Result<Vec<InboxItemValue>>;
}
