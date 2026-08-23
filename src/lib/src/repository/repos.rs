use dsot_model::{InboxStatus, InboxValue};
use uuid::Uuid;

use crate::{
    error::{DsotError, Result},
    state::{
        devices::RemoteDevice,
        inbox::{InboxFilter, InboxItemValue},
    },
};

declare_repository!(
    Device {
        async fn list_devices(&self) -> Result<Vec<RemoteDevice>>; Ok(vec![]);
        async fn add_device(&self, device: RemoteDevice) -> Result<bool>; Ok(false);
        async fn remove_device(&self, id: iroh::EndpointId) -> Result<()>; Ok(());
    },
    Inbox {
        async fn load_inbox(&self, filter: &InboxFilter) -> Result<Vec<InboxItemValue>>; Ok(vec![]);
        async fn add_inbox_item(&self, value: InboxValue) -> Result<()>; Ok(());
        async fn remove_inbox_item(&self, id: Uuid) -> Result<bool>; Ok(false);
        async fn update_inbox_status(&self, id: Uuid, status: InboxStatus) -> Result<bool>; Ok(false);
        async fn get_inbox_item(&self, id: Uuid) -> Result<InboxItemValue>; Err(DsotError::FeatureNotAvailable);
    },
    User {
        async fn list_users(&self) -> Result<Vec<String>>; Ok(vec![]);
    }
);
