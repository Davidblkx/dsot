use std::path::PathBuf;

use dsot_db_sync::DatabaseManager;

pub struct UserManager {
    dir: PathBuf,
}

impl UserManager {
    pub fn open<T: Into<PathBuf>>(root: T) -> Result<Self, std::io::Error> {
        let dir = root.into().join("users");

        if !dir.exists() {
            log::info!("Creating users directory: {}", dir.display());
            std::fs::create_dir_all(&dir)?;
        }

        Ok(Self { dir })
    }

    pub fn list_users(&self) -> Result<Vec<String>, std::io::Error> {
        let mut list = Vec::new();

        for r in std::fs::read_dir(&self.dir)? {
            let entry = r?;
            if entry.path().is_dir() {
                match entry.file_name().into_string() {
                    Ok(value) => list.push(value),
                    _ => {
                        log::warn!(
                            "Skipping non-string user directory: {}",
                            entry.file_name().display()
                        );
                    }
                };
            }
        }

        Ok(list)
    }

    pub fn open_user_db(&self, user: &str) -> dsot_db_sync::manager::Result<DatabaseManager> {
        DatabaseManager::open_folder(self.dir.join(user))
    }
}
