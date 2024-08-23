mod setup_config;
mod instance;

pub use setup_config::*;
pub use instance::*;

use crate::error::Result;
use crate::db::LocalDBManager;

pub fn setup(config: SetupConfig) -> Result<DSOTInstance> {
    if !config.local_data.exists() {
        log::trace!("Creating local data directory: {:?}", config.local_data);
        std::fs::create_dir_all(&config.local_data)?;
    }

    let mut db_manager = LocalDBManager::new(&config.local_data.as_path());

    db_manager.create_or_update(&config.lib.name)?;

    let p = db_manager.get_path(&config.lib.name);
    log::trace!("Loading database from: {:?}", p);

    Ok(DSOTInstance {
        db_manager,
        library: config.lib.name,
    })
}
