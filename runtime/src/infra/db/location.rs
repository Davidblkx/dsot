use std::path::PathBuf;
use std::str::FromStr;

use sqlx::sqlite::SqliteConnectOptions;

use crate::Config;
use crate::error::Result;

pub static DB_NAME: &str = "runtime.db";

pub async fn connect_db_file(cfg: &Config) -> Result<sqlx::SqlitePool> {
    let path = cfg.data_location.join(DB_NAME);
    log::trace!("Loading database from: {}", path.display());

    if !path.exists() {
        log::trace!("Database file does not exist, it will be created");
    }

    let conn_str = format!("sqlite://{}", path.display());
    let conn = SqliteConnectOptions::from_str(&conn_str)?.create_if_missing(true);
    log::debug!(
        "Connecting to database with connection string: {}",
        conn_str
    );

    let pool = sqlx::SqlitePool::connect_with(conn).await?;

    Ok(pool)
}

pub fn create_backup(cfg: &Config, daily_backup: bool) -> Result<Option<PathBuf>> {
    let backup_path = cfg.data_location.join("db_backup");
    if !backup_path.exists() {
        log::trace!("Creating backup directory at: {}", backup_path.display());
        std::fs::create_dir_all(&backup_path)?;
    }

    let dbpath = cfg.data_location.join(DB_NAME);
    if dbpath.metadata()?.len() <= 0 {
        log::trace!(
            "Database file is empty or does not exist: {}, skipping backup.",
            dbpath.display()
        );
        return Ok(None);
    }

    // Create suffix with current timestamp
    let timestamp = if daily_backup {
        chrono::Local::now().format("%Y%m%d")
    } else {
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    };
    let backup_file = backup_path.join(format!("{}__{}.BAK", DB_NAME, timestamp));

    if backup_file.exists() {
        log::trace!("Backup file already exists: {}", backup_file.display());
    } else {
        log::trace!("Creating backup file: {}", backup_file.display());
        std::fs::copy(dbpath, &backup_file)?;
    }

    Ok(Some(backup_file))
}
