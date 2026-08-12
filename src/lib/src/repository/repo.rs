use iroh::EndpointId;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::Repository;
use crate::{
    core::{DsotCore, config::DsotAppConfig, init::DsotCoreInitOptions},
    error::Result,
    repository::{DeviceRepository, InboxRepository, UserRepository},
    state::{
        devices::RemoteDevice,
        inbox::{InboxFilter, InboxItemValue},
    },
    user::DsotUser,
};

#[derive(Debug, Clone)]
pub struct DsotRepository {
    repo: Arc<RwLock<Box<dyn Repository>>>,
}

impl DsotRepository {
    pub fn new(repo: impl Repository + 'static) -> Self {
        Self {
            repo: Arc::new(RwLock::new(Box::new(repo))),
        }
    }

    pub async fn init(
        options: &DsotCoreInitOptions,
        config: &DsotAppConfig,
        user: DsotUser,
    ) -> Result<Self> {
        if options.cap.can_disk_access() {
            let repo = super::local::LocalRepo::init(config.data_dir.clone(), user);
            Ok(DsotRepository::new(repo))
        } else {
            let repo = super::noop::NoopRepo::init();
            Ok(DsotRepository::new(repo))
        }
    }

    async fn set_repo(&self, repo: impl Repository + 'static) {
        let repo = Box::new(repo);
        let mut writer = self.repo.write().await;
        *writer = repo;
    }
}

#[async_trait::async_trait]
impl DeviceRepository for DsotRepository {
    async fn list_devices(&self) -> Result<Vec<RemoteDevice>> {
        self.repo.read().await.list_devices().await
    }
    async fn add_device(&self, _device: RemoteDevice) -> Result<bool> {
        self.repo.write().await.add_device(_device).await
    }
    async fn remove_device(&self, _id: iroh::EndpointId) -> Result<()> {
        self.repo.write().await.remove_device(_id).await
    }
}

#[async_trait::async_trait]
impl UserRepository for DsotRepository {
    async fn list_users(&self) -> Result<Vec<String>> {
        self.repo.read().await.list_users().await
    }
}

#[async_trait::async_trait]
impl InboxRepository for DsotRepository {
    async fn load_inbox(&self, filter: &InboxFilter) -> Result<Vec<InboxItemValue>> {
        self.repo.read().await.load_inbox(filter).await
    }

    async fn add_inbox_item(&self, value: ::dsot_model::InboxValue) -> Result<()> {
        self.repo.write().await.add_inbox_item(value).await
    }

    async fn remove_inbox_item(&self, id: uuid::Uuid) -> Result<bool> {
        self.repo.write().await.remove_inbox_item(id).await
    }
    async fn update_inbox_status(
        &self,
        id: uuid::Uuid,
        status: ::dsot_model::InboxStatus,
    ) -> Result<bool> {
        self.repo
            .write()
            .await
            .update_inbox_status(id, status)
            .await
    }
}

impl super::Repository for DsotRepository {}

impl DsotCore {
    pub async fn connect_remote_repo(&self, id: EndpointId) -> Result<()> {
        let device = self.net.connect_remote_device(id).await?;
        let repo = super::remote::RemoteRepo::init(self, &device);
        self.repo.set_repo(repo).await;
        Ok(())
    }
}
