use crate::error::{Result, RuntimeError};

pub fn init_folder(path: &str) -> Result<()> {
    let exists = std::fs::exists(path).map_err(RuntimeError::IOError)?;

    if !exists {
        log::debug!("Creating data folder at: {}", path);
        std::fs::create_dir_all(path).map_err(RuntimeError::IOError)?;
    }

    Ok(())
}
