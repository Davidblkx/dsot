use sqlx::migrate::Migrator;

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

/// Checks if there are any pending migrations in the database.
pub async fn has_pending_migrations(pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
    let has_table = sqlx::query_scalar!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='_sqlx_migrations'"
    )
    .fetch_one(pool)
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
        .fetch_one(pool)
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

/// Runs all pending migrations in the database.
pub async fn run_pending_migrations(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    log::info!("Running pending migrations...");

    MIGRATOR.run(pool).await?;

    log::info!("All migrations have been successfully applied.");

    Ok(())
}
