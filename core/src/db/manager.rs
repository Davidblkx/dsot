use std::{collections::HashMap, path::{Path, PathBuf}, sync::Mutex};

use crate::error::{Result, DsotError};
use super::{local_db::ConnectionGuard, LocalDB};

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

    pub fn get_path<'a>(&'a self) -> &'a PathBuf {
        &self.path
    }

    fn build_db_path(&self, name: &str) -> Result<String> {
        let mut p = self.path.clone();
        p.push(format!("{}.db3", name));

        p.into_os_string().to_str().map(|s| s.to_string())
            .ok_or(DsotError::InvalidOSString)
    }

    pub fn get(&self, name: &str) -> Result<ConnectionGuard> {
        let lock = match self.locked.lock() {
            Ok(e) => e,
            Err(p) => p.into_inner()
        };

        if !self.cache.contains_key(name) {
            return Err(DsotError::DatabaseNotFound(name.to_string()));
        }

        let db = self.cache.get(name).unwrap().lock()?;
        drop(lock);
        Ok(db)
    }

    pub fn create(&mut self, name: &str) -> Result<()> {
        let lock = match self.locked.lock() {
            Ok(e) => e,
            Err(p) => p.into_inner()
        };
        let db_path = self.build_db_path(name)?;

        if self.cache.contains_key(name) {
            return Err(DsotError::DuplicatedInitialization(name.to_string()));
        }

        let db = LocalDB::new(&db_path);

        // TODO: apply migrations and what not

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
                manager.create(&name).unwrap()
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

        assert!(manager.create(&name).is_ok());

        match manager.create(&name) {
            Ok(_) => assert!(false),
            Err(e) => match e {
                DsotError::DuplicatedInitialization(err_name) => assert_eq!(err_name, name),
                _ => assert!(false)
            },
        }

        // std::fs::remove_file(format!("./{}.db3", &name)).unwrap();
    }
}
