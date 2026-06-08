use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use super::{DatabaseManager, DatabaseManagerError, Result};
use crate::DsotDatabase;

pub static DB_NAME: &'static str = "library.sqlite";
pub static JOURNAL_NAME: &'static str = "library.journal";

impl DatabaseManager {
    pub async fn open_database(&self) -> Result<DsotDatabase> {
        log::info!("Opening sqlite database at {}", self.get_db_path());
        let db_options = SqliteConnectOptions::new()
            .filename(self.get_db_path())
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(db_options)
            .await?;

        log::debug!("Running migrations");
        sqlx::migrate!("../../../migrations").run(&pool).await?;

        log::info!("Opening journal at {}", self.get_journal_path());
        let journal = redb::Database::create(self.get_journal_path())?;

        let id = self
            .dir
            .file_name()
            .ok_or(DatabaseManagerError::PathIsNotAFolder)?
            .to_string_lossy()
            .into_owned();

        Ok(DsotDatabase {
            journal,
            sql: pool,
            id,
        })
    }

    pub fn get_db_path(&self) -> String {
        format!("{}/{}", self.dir.display(), DB_NAME)
    }

    pub fn get_journal_path(&self) -> String {
        format!("{}/{}", self.dir.display(), JOURNAL_NAME)
    }
}
