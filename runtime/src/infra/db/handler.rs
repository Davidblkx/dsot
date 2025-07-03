use std::path::PathBuf;

use dashmap::DashMap;

use crate::error::Result;

use super::conn::{create_db_connection, create_trx_from_pool, create_trx_from_ref};
use super::kind::DbKind;

/// DatabaseHandler is responsible for managing the main database and user-specific databases.
/// It's the main entry point for database operations in the runtime.
pub struct DatabaseHandler {
    pub db: sqlx::SqlitePool,
    users_db: DashMap<String, sqlx::SqlitePool>,
    db_path: PathBuf,
    daily_backup: bool,
}

impl DatabaseHandler {
    pub async fn new(config: &crate::Config) -> Result<Self> {
        let daily_backup = true;

        let db = create_db_connection(&config.data_location, &DbKind::Main, daily_backup).await?;

        Ok(DatabaseHandler {
            db,
            users_db: DashMap::new(),
            db_path: config.data_location.clone(),
            daily_backup: true,
        })
    }

    /// Creates a new user-specific database transaction.
    pub async fn create_user_transaction(&self, user_id: &str) -> Result<sqlx::SqliteTransaction> {
        if let Some(pool) = self.users_db.get(user_id) {
            let trx = create_trx_from_ref(pool).await?;
            return Ok(trx);
        }

        let user_db =
            create_db_connection(&self.db_path, &DbKind::User(user_id), self.daily_backup).await?;
        let trx = create_trx_from_pool(&user_db).await?;

        self.users_db.insert(user_id.to_string(), user_db);

        Ok(trx)
    }

    /// Creates a new transaction on the main database connection.
    pub async fn create_transaction(&self) -> Result<sqlx::SqliteTransaction> {
        let trx = create_trx_from_pool(&self.db).await?;
        Ok(trx)
    }
}
