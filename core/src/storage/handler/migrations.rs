use core::result::Result::Ok;

use sqlx::{SqlitePool, migrate::Migrator};

use super::SqliteDbHandler;
use crate::error::{DsotError, Result};

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

impl SqliteDbHandler {
    /// Checks if database has any missing migration
    pub async fn has_pending_migrations(&self) -> Result<bool> {
        if let Some(db_pool) = &self.db_pool {
            return has_pending_migrations(db_pool).await;
        }

        DsotError::SqlClosedConnection.to_err()
    }

    /// Runs all pending migrations in the database.
    pub async fn run_pending_migrations(&self) -> Result<()> {
        if let Some(db_pool) = &self.db_pool {
            run_pending_migrations(db_pool).await?;
            return Ok(());
        }

        DsotError::SqlClosedConnection.to_err()
    }
}

async fn run_pending_migrations(db_pool: &SqlitePool) -> Result<()> {
    log::info!("Running pending migrations...");

    MIGRATOR.run(db_pool).await?;

    log::info!("All migrations have been successfully applied.");

    Ok(())
}

async fn has_pending_migrations(db_pool: &SqlitePool) -> Result<bool> {
    let has_table = sqlx::query_scalar!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='_sqlx_migrations'"
    )
    .fetch_one(db_pool)
    .await
    .unwrap_or(None)
    .is_some();

    if !has_table {
        log::debug!("No migration table found, migrations are pending.");
        return Ok(true);
    }

    for m in MIGRATOR.iter() {
        let has_migration = sqlx::query_scalar!(
            "SELECT version FROM _sqlx_migrations WHERE version = ?",
            m.version
        )
        .fetch_one(db_pool)
        .await
        .unwrap_or(None)
        .is_some();

        if !has_migration {
            log::debug!("Pending migration found: {}", m.version);
            return Ok(true);
        }
    }

    Ok(false)
}
