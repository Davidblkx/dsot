use iroh::EndpointId;

use super::DsotRepository;
use crate::{
    core::{DsotCore, config::DsotAppConfig, init::DsotCoreInitOptions},
    error::Result,
    user::DsotUser,
};

impl DsotRepository {
    pub async fn init(
        options: &DsotCoreInitOptions,
        config: &DsotAppConfig,
        user: DsotUser,
    ) -> Result<Self> {
        if options.cap.can_disk_access() {
            let repo = super::local::LocalRepo::init(config.data_dir.clone(), user);
            Ok(DsotRepository::new(repo))
        } else {
            let repo = super::repos::DefaultRepository::default();
            Ok(DsotRepository::new(repo))
        }
    }
}

impl DsotCore {
    pub async fn connect_remote_repo(&self, id: EndpointId) -> Result<()> {
        let device = self.net.connect_remote_device(id).await?;
        let repo = super::remote::RemoteRepo::init(self, &device);
        self.repo.set_repo(repo).await;
        Ok(())
    }
}
