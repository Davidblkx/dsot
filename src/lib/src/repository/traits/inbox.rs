use async_trait::async_trait;
use dsot_model::{InboxStatus, InboxValue};
use uuid::Uuid;

use crate::{
    error::Result,
    state::inbox::{InboxFilter, InboxItemValue},
};

#[async_trait]
pub trait InboxRepository {
    async fn load_inbox(&self, filter: &InboxFilter) -> Result<Vec<InboxItemValue>>;
    async fn add_inbox_item(&self, value: InboxValue) -> Result<()>;
    async fn remove_inbox_item(&self, id: Uuid) -> Result<bool>;
    async fn update_inbox_status(&self, id: Uuid, status: InboxStatus) -> Result<bool>;
}
