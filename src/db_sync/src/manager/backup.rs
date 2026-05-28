use std::collections::HashSet;
use std::path::PathBuf;

use uuid::Uuid;

use super::load::{DB_NAME, JOURNAL_NAME};
use super::{DatabaseManager, Result};

static BACKUP_FOLDER: &'static str = "backups";

#[derive(Debug)]
pub struct DatabaseBackup {
    pub id: Uuid,
    pub root: PathBuf,
}

impl DatabaseBackup {
    pub fn create(root: PathBuf) -> Result<Self> {
        let id = Uuid::now_v7();
        let bck = Self { id, root };

        std::fs::create_dir_all(bck.root.join(BACKUP_FOLDER))?;
        std::fs::copy(bck.get_db_path(), bck.get_backup_db_path())?;
        std::fs::copy(bck.get_journal_path(), bck.get_backup_journal_path())?;

        Ok(bck)
    }

    pub fn restore(&self) -> Result<()> {
        std::fs::copy(self.get_backup_db_path(), self.get_db_path())?;
        std::fs::copy(self.get_backup_journal_path(), self.get_journal_path())?;

        Ok(())
    }

    pub fn list_backups(root: &PathBuf) -> Vec<DatabaseBackup> {
        let bck_folder = root.join(BACKUP_FOLDER);

        let mut ids = HashSet::new();

        if let Ok(entries) = std::fs::read_dir(bck_folder) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if let Some(id) = file_name.rsplit_once("__").map(|(_, after)| after) {
                        ids.insert(Uuid::parse_str(id));
                    }
                }
            }
        }

        let mut bck_list = Vec::new();

        for id in ids.into_iter().flatten() {
            let bck = DatabaseBackup {
                id,
                root: root.clone(),
            };
            if bck.is_valid() {
                bck_list.push(bck);
            }
        }

        bck_list
    }

    pub fn get_backup_db_path(&self) -> PathBuf {
        self.root
            .join(BACKUP_FOLDER)
            .join(format!("{}__{}", DB_NAME, self.id))
    }

    pub fn get_backup_journal_path(&self) -> PathBuf {
        self.root
            .join(BACKUP_FOLDER)
            .join(format!("{}__{}", JOURNAL_NAME, self.id))
    }

    pub fn get_db_path(&self) -> PathBuf {
        self.root.join(DB_NAME)
    }

    pub fn get_journal_path(&self) -> PathBuf {
        self.root.join(JOURNAL_NAME)
    }

    pub fn is_valid(&self) -> bool {
        self.get_backup_db_path().exists() && self.get_backup_journal_path().exists()
    }
}

impl DatabaseManager {
    pub fn create_backup(&self) -> Result<DatabaseBackup> {
        let bck = DatabaseBackup::create(self.dir.clone())?;
        Ok(bck)
    }

    pub fn get_backups(&self) -> Vec<DatabaseBackup> {
        DatabaseBackup::list_backups(&self.dir)
    }
}
