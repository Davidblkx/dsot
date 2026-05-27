use std::path::PathBuf;

use super::load::{DB_NAME, JOURNAL_NAME};
use super::{DatabaseManager, Result};

static BACKUP_FOLDER: &'static str = "backups";

impl DatabaseManager {
    pub fn backup(&self) -> Result<()> {
        let db = PathBuf::from(self.get_db_path());
        let jrn = PathBuf::from(self.get_journal_path());

        if !db.exists() || !jrn.exists() {
            return Ok(());
        }

        let suffix = uuid::Uuid::now_v7().to_string();

        let db_backup = PathBuf::from(format!("{}/{}__{}", BACKUP_FOLDER, DB_NAME, suffix));
        let jrn_backup = PathBuf::from(format!("{}/{}__{}", BACKUP_FOLDER, JOURNAL_NAME, suffix));

        std::fs::create_dir_all(BACKUP_FOLDER)?;
        std::fs::copy(&db, &db_backup)?;
        std::fs::copy(&jrn, &jrn_backup)?;

        Ok(())
    }
}
