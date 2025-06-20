use std::path::PathBuf;
use std::str::FromStr;

use sqlx::sqlite::SqliteConnectOptions;

use crate::error::Result;

pub static DB_NAME: &str = "runtime.db";

pub enum DbType<'a> {
    Main,
    User(&'a str),
}

impl DbType<'_> {
    pub fn create_for_user<'a>(user_id: &'a str) -> DbType<'a> {
        DbType::User(user_id)
    }
}

fn get_db_path<'a>(path: &PathBuf, db_type: &DbType<'a>) -> PathBuf {
    match db_type {
        DbType::Main => path.join(DB_NAME),
        DbType::User(user_id) => path.join(format!("user_{}.db", user_id)),
    }
}

pub async fn connect_db<'a>(path: &PathBuf, db_type: &DbType<'a>) -> Result<sqlx::SqlitePool> {
    let db_path = get_db_path(path, db_type);
    log::trace!("Loading database from: {}", db_path.display());

    if !db_path.exists() {
        log::trace!("Database file does not exist, it will be created");
    }
    let conn_str = format!("sqlite://{}", db_path.display());
    let conn = SqliteConnectOptions::from_str(&conn_str)?.create_if_missing(true);
    log::debug!(
        "Connecting to database with connection string: {}",
        conn_str
    );

    let pool = sqlx::SqlitePool::connect_with(conn).await?;
    Ok(pool)
}

pub fn create_backup<'a>(
    path: &PathBuf,
    db_type: &DbType<'a>,
    daily_backup: bool,
) -> Result<Option<PathBuf>> {
    let backup_path = path.join(DB_NAME);
    if !backup_path.exists() {
        log::trace!("Creating backup directory at: {}", backup_path.display());
        std::fs::create_dir_all(&backup_path)?;
    }

    let dbpath = get_db_path(&backup_path, db_type);
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
