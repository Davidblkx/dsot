use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use dsot_core::storage::SqliteDbHandler;
use std::path::PathBuf;

use crate::Config;
use crate::error::Result;

use super::DatabaseHandlerError;

static LIBRARY_NAME: &'static str = "__library";

pub struct DatabaseHandler {
    pub dbs: DashMap<String, SqliteDbHandler>,
    pub data_folder: PathBuf,
    pub backup_folder: PathBuf,
}

impl DatabaseHandler {
    pub async fn new(config: &Config) -> Result<Self> {
        let data_folder = config.data_location.clone();
        let backup_folder = data_folder.join("backup");

        let v = Self {
            dbs: DashMap::with_capacity(10),
            data_folder,
            backup_folder,
        };

        v.create_db(LIBRARY_NAME.to_string(), "library").await?;

        Ok(v)
    }

    pub fn get_lib_db(&self) -> Result<Ref<'_, String, SqliteDbHandler>> {
        self.dbs
            .get(LIBRARY_NAME)
            .ok_or(DatabaseHandlerError::LibraryNotDefined.into())
    }

    pub fn get_lib_db_mut(&self) -> Result<RefMut<'_, String, SqliteDbHandler>> {
        self.dbs
            .get_mut(LIBRARY_NAME)
            .ok_or(DatabaseHandlerError::LibraryNotDefined.into())
    }

    pub async fn get_user_db(&self, user: &str) -> Result<Ref<'_, String, SqliteDbHandler>> {
        let user_key = format!("User.{}", user);
        self.create_db(user_key.clone(), user).await?;

        self.dbs
            .get(&user_key)
            .ok_or(DatabaseHandlerError::UserNotDefined(user_key).into())
    }

    pub async fn get_user_db_mut(&self, user: &str) -> Result<RefMut<'_, String, SqliteDbHandler>> {
        let user_key = format!("User.{}", user);
        self.create_db(user_key.clone(), user).await?;

        self.dbs
            .get_mut(&user_key)
            .ok_or(DatabaseHandlerError::UserNotDefined(user_key).into())
    }

    pub async fn backup(&self) -> Result<()> {
        for mut handler in self.dbs.iter_mut() {
            handler.backup().await?;
        }

        Ok(())
    }

    async fn create_db(&self, key: String, name: &str) -> Result<()> {
        if self.dbs.contains_key(&key) {
            log::warn!("Database for key {}, already created", key);
            return Ok(());
        }

        log::debug!("Creating database for {}", key);

        let mut db =
            SqliteDbHandler::new_file_with_backup(name, &self.data_folder, &self.backup_folder);
        db.open().await?;

        if db.has_pending_migrations().await? {
            if let Some(db_path) = db.connection_kind.get_db_path() {
                if db_path.exists() {
                    db.backup().await?;
                }
            }

            db.run_pending_migrations().await?;
        }

        self.dbs.insert(key, db);

        Ok(())
    }
}
