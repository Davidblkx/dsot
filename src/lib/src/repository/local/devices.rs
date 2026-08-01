use async_trait::async_trait;
use std::path::PathBuf;

use crate::{error::Result, repository::DeviceRepository, state::devices::RemoteDevice};

static FILE_NAME: &'static str = "devices.toml";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct DevicesFile {
    pub devices: Vec<RemoteDevice>,
}
dsot_serde::serde_text!(DevicesFile);

#[derive(Debug)]
pub struct LocalDeviceRepo {
    file: PathBuf,
}

impl LocalDeviceRepo {
    pub fn new(root: PathBuf) -> Self {
        Self {
            file: root.join(FILE_NAME),
        }
    }

    fn load(&self) -> Result<DevicesFile> {
        if self.file.exists() {
            let devices = DevicesFile::from_file(self.file.clone())?;
            Ok(devices)
        } else {
            Ok(DevicesFile { devices: vec![] })
        }
    }

    fn save(&self, devices: DevicesFile) -> Result<()> {
        devices.to_file(self.file.clone())?;
        Ok(())
    }
}

#[async_trait]
impl DeviceRepository for LocalDeviceRepo {
    async fn list_devices(&self) -> Result<Vec<RemoteDevice>> {
        let file = self.load()?;
        Ok(file.devices)
    }

    async fn add_device(&self, device: RemoteDevice) -> Result<bool> {
        let mut file = self.load()?;

        for d in file.devices.iter() {
            if device.id == d.id {
                return Ok(false);
            }
        }

        file.devices.push(device);
        self.save(file)?;
        Ok(true)
    }
    async fn remove_device(&self, id: iroh::EndpointId) -> Result<()> {
        let mut file = self.load()?;
        file.devices.retain(|d| d.id != id);
        self.save(file)
    }
}
