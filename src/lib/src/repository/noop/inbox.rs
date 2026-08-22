use async_trait::async_trait;
use dsot_db_sync::DBSyncError;
use dsot_model::{InboxStatus, InboxValue};
use uuid::Uuid;

use crate::{
    error::{DsotError, Result},
    repository::InboxRepository,
    state::inbox::{InboxFilter, InboxItemValue},
};

#[derive(Debug)]
pub struct NoopInboxRepository;

#[async_trait]
impl InboxRepository for NoopInboxRepository {
    async fn load_inbox(&self, _filter: &InboxFilter) -> Result<Vec<InboxItemValue>> {
        Ok(vec![])
    }
    async fn add_inbox_item(&self, _value: InboxValue) -> Result<()> {
        Ok(())
    }
    async fn remove_inbox_item(&self, _id: Uuid) -> Result<bool> {
        Ok(true)
    }
    async fn update_inbox_status(&self, _id: Uuid, _status: InboxStatus) -> Result<bool> {
        Ok(true)
    }
    async fn get_inbox_item(&self, _id: Uuid) -> Result<InboxItemValue> {
        Err(DsotError::DatabaseSyncError(DBSyncError::NoOpenConnection))
    }
}
