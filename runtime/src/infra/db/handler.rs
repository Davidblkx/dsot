use std::path::PathBuf;

use dashmap::DashMap;
use sqlx::SqlitePool;

use super::location::DbType;
use crate::error::Result;

pub struct DatabaseHandler {
    pub db: sqlx::SqlitePool,
    users_db: DashMap<String, sqlx::SqlitePool>,
    db_path: PathBuf,
    daily_backup: bool,
}

impl DatabaseHandler {
    pub async fn new(config: &crate::Config) -> Result<Self> {
        let db_path = config.data_location.clone();
        let daily_backup = true;

        let db =
            DatabaseHandler::initialize_database(&db_path, &DbType::Main, daily_backup).await?;

        Ok(DatabaseHandler {
            db,
            users_db: DashMap::new(),
            db_path: config.data_location.clone(),
            daily_backup: true,
        })
    }

    async fn initialize_database<'a>(
        path: &PathBuf,
        db_type: &DbType<'a>,
        daily_backup: bool,
    ) -> crate::error::Result<SqlitePool> {
        let pool = super::location::connect_db(path, &db_type).await?;

        DatabaseHandler::apply_migrations(&pool).await?;

        Ok(pool)
    }

    async fn apply_migrations(db: &SqlitePool) -> crate::error::Result<()> {
        if super::migrations::has_pending_migrations(db).await? {
            super::migrations::run_pending_migrations(db).await?;
        } else {
            log::debug!("No pending migrations found.");
        }
        Ok(())
    }

    pub async fn get_user_db(&self, user_id: &str) -> Result<sqlx::SqliteTransaction> {
        if let Some(pool) = self.users_db.get(user_id) {
            return Ok(pool.begin().await?);
        }

        let user_db =
            super::location::connect_db(&self.db_path, &DbType::create_for_user(user_id)).await?;
        let trx = user_db.begin().await?;

        self.users_db.insert(user_id.to_string(), user_db);

        Ok(trx)
    }
}
