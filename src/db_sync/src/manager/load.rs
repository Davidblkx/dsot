use sqlx::sqlite::SqlitePoolOptions;

use super::{DatabaseManager, Result};
use crate::DsotDatabase;

static DB_NAME: &'static str = "library.sqlite";
static JOURNAL_NAME: &'static str = "library.journal";

impl DatabaseManager {
    pub async fn open_database(&self) -> Result<DsotDatabase> {
        let db_connect_str = format!("sqlite://{}", self.get_db_path());

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(&db_connect_str)
            .await?;

        sqlx::migrate!("../../migrations").run(&pool).await?;

        let journal = redb::Database::create(self.get_journal_path())?;

        Ok(DsotDatabase { journal, sql: pool })
    }

    pub fn get_db_path(&self) -> String {
        format!("{}/{}", self.dir.display(), DB_NAME)
    }

    pub fn get_journal_path(&self) -> String {
        format!("{}/{}", self.dir.display(), JOURNAL_NAME)
    }
}
