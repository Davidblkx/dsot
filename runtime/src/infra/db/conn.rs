use super::error::DbError;
use super::kind::DbKind;
use super::migrations::{has_pending_migrations, run_pending_migrations};
use super::path::{create_db_backup_path, get_db_path};

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::PathBuf;
use std::str::FromStr;

/// Helper function to create a new transaction on the provided database connection.
pub async fn create_trx_from_ref<'a>(
    pool_ref: dashmap::mapref::one::Ref<'a, String, sqlx::SqlitePool>,
) -> Result<sqlx::SqliteTransaction<'static>, DbError> {
    log::trace!("Starting a new transaction on the database connection.");
    let transaction = pool_ref.begin().await?;
    log::trace!("Transaction started successfully.");
    Ok(transaction)
}

/// Helper function to create a new transaction on the provided database connection.
pub async fn create_trx_from_pool(
    pool_ref: &sqlx::SqlitePool,
) -> Result<sqlx::SqliteTransaction<'static>, DbError> {
    log::trace!("Starting a new transaction on the database connection.");
    let transaction = pool_ref.begin().await?;
    log::trace!("Transaction started successfully.");
    Ok(transaction)
}

/// Creates a new database connection based on the provided data path and database kind.
/// If `daily_backup` is true, it will create a daily backup of the database.
/// If there are pending migrations, it will create a backup before running them.
/// Returns a `SqlitePool` if successful, or an error if the connection fails.
pub async fn create_db_connection<'a>(
    data_path: &PathBuf,
    kind: &DbKind<'a>,
    daily_backup: bool,
) -> Result<SqlitePool, DbError> {
    let db_path = get_db_path(data_path, kind);
    let mut pool = connect_db(&db_path).await?;
    let has_migrations = has_pending_migrations(&pool).await?;

    if daily_backup {
        log::trace!(
            "Creating daily backup for database at: {}",
            db_path.display()
        );
        create_backup(pool, data_path, kind, true).await?;
        pool = connect_db(&db_path).await?;
    }

    if has_migrations {
        create_backup(pool, data_path, kind, false).await?;
        pool = connect_db(&db_path).await?;
        run_pending_migrations(&pool).await?;
    } else {
        log::trace!("No pending migrations found.");
    }

    Ok(pool)
}

async fn connect_db(path: &PathBuf) -> Result<SqlitePool, DbError> {
    log::trace!("Connecting to database at: {}", path.display());
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

/// Ensures connection is closed, copies the database file to a backup location.
async fn create_backup<'a>(
    pool: SqlitePool,
    data_path: &PathBuf,
    kind: &DbKind<'a>,
    daily_backup: bool,
) -> Result<(), DbError> {
    if !pool.is_closed() {
        pool.close().await;
    }

    let db_path = get_db_path(data_path, kind);
    if !db_path.exists() {
        log::trace!("Database file does not exist, skipping backup.");
        return Ok(());
    }

    // Create backup path
    match create_db_backup_path(data_path, kind, daily_backup) {
        Ok(backup_path) => {
            log::trace!("Creating backup at: {}", backup_path.display());
            std::fs::copy(&db_path, &backup_path)?;
            log::info!("Backup created successfully at: {}", backup_path.display());
            Ok(())
        }
        Err(e) => match e {
            DbError::BackupExists(path_str) => {
                if daily_backup {
                    // If daily backup is enabled, we can skip the backup if it already exists
                    Ok(())
                } else {
                    log::trace!("Backup already exists: {}", path_str);
                    Err(DbError::BackupExists(path_str))
                }
            }
            _ => {
                log::trace!("Failed to create backup: {}", e);
                Err(e)
            }
        },
    }
}
