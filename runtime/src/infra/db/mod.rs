use sqlx::SqlitePool;

use super::Config;
use crate::error::Result;

mod location;
mod migrations;

pub async fn initialize_database(cfg: &Config) -> Result<SqlitePool> {
    let pool = location::connect_db_file(cfg).await?;

    if migrations::has_pending_migrations(&pool).await? {
        location::create_backup(cfg, false)?;
        migrations::run_pending_migrations(&pool).await?;
    } else {
        log::debug!("No pending migrations found.");
    }

    Ok(pool)
}
