use std::{borrow::BorrowMut, collections::HashMap, path::{Path, PathBuf}, sync::Mutex};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::error::{Result, DsotError};
use super::{local_db::ConnectionGuard, LocalDB};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct LocalDBManager {
    path: PathBuf,
    cache: HashMap<String, LocalDB>,
    locked: Mutex<()>,
}

impl LocalDBManager {
    pub fn new(path: &Path) -> Self {
        LocalDBManager {
            path: path.to_path_buf(),
            cache: HashMap::new(),
            locked: Mutex::new(())
        }
    }

    pub fn get_path(&self, name: &str) -> PathBuf {
        let mut p = self.path.clone();
        p.push(format!("{}.db3", name));
        p
    }

    fn build_db_path(&self, name: &str) -> Result<String> {
        self.get_path(name)
            .into_os_string()
            .to_str()
            .map(|s| s.to_string())
            .ok_or(DsotError::InvalidOSString)
    }

    pub fn get(&self, name: &str) -> Result<ConnectionGuard> {
        let lock = match self.locked.lock() {
            Ok(e) => e,
            Err(p) => p.into_inner()
        };

        if !self.cache.contains_key(name) {
            log::warn!("Database not found: {}", name);
            return Err(DsotError::DatabaseNotFound(name.to_string()));
        }

        let db = self.cache.get(name).unwrap().lock()?;
        drop(lock);
        Ok(db)
    }

    pub fn create_or_update(&mut self, name: &str) -> Result<()> {
        let lock = match self.locked.lock() {
            Ok(e) => e,
            Err(p) => p.into_inner()
        };
        let db_path = self.build_db_path(name)?;

        if self.cache.contains_key(name) {
            log::warn!("Database already initialized: {}", name);
            return Err(DsotError::DatabaseDuplicatedInit(name.to_string()));
        }

        let db = LocalDB::new(&db_path);

        let mut guard = db.lock()?;

        log::trace!("Running migrations for: {}", name);
        match guard.connection.borrow_mut().run_pending_migrations(MIGRATIONS) {
            Ok(_) => {
                log::trace!("Migrations completed for: {}", name);
            },
            Err(e) => {
                drop(guard);
                let message = format!("Migration failed: {}", e);
                log::warn!("{}", message);
                return Err(DsotError::DatabaseMigrationError(message));
            }
        }
        drop(guard);

        self.cache.insert(name.to_string(), db);

        drop(lock);
        Ok(())
    }
}

unsafe impl Sync for LocalDBManager {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_multithread() {
        let mut manager = LocalDBManager::new(Path::new("./"));
        let name = "manager_is_multithread".to_string();

        std::thread::scope(|s| {
            s.spawn(|| {
                manager.create_or_update(&name).unwrap()
            });
        });

        let db = manager.get(&name).unwrap();
        drop(db);
        std::fs::remove_file(format!("./{}.db3", &name)).unwrap();
    }

    #[test]
    fn fail_when_not_created() {
        let manager = LocalDBManager::new(Path::new("./"));
        let name = "fail_when_not_created".to_string();

        match manager.get(&name) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                DsotError::DatabaseNotFound(err_name) => assert_eq!(err_name, name),
                _ => assert!(false)
            },
        };
    }

    #[test]
    fn fail_when_duplicated() {
        let mut manager = LocalDBManager::new(Path::new("./"));
        let name = "fail_when_duplicated".to_string();

        assert!(manager.create_or_update(&name).is_ok());

        match manager.create_or_update(&name) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                DsotError::DatabaseDuplicatedInit(err_name) => assert_eq!(err_name, name),
                _ => assert!(false)
            },
        }

        std::fs::remove_file(format!("./{}.db3", &name)).unwrap();
    }
}
