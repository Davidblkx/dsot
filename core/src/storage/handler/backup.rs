use std::ffi::OsStr;

use crate::{error::Result, storage::error::DatabaseHandlerError};

use super::SqliteDbHandler;

impl SqliteDbHandler {
    /// Backup database and related journal to a folder
    pub async fn backup(&mut self) -> Result<uuid::Uuid> {
        let should_open = self.is_open();
        self.close().await?;

        self.connection_kind.ensure_folders()?;
        let backup_id = uuid::Uuid::now_v7();

        let id_str = SqliteDbHandler::get_backup_id_string(&backup_id);

        let src_db = self
            .connection_kind
            .get_db_path()
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;
        let bak_db = self
            .connection_kind
            .get_backup_db_path(&id_str)
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;

        let src_jrn = self
            .connection_kind
            .get_journal_path()
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;
        let bak_jrn = self
            .connection_kind
            .get_backup_journal_path(&id_str)
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;

        log::trace!(
            "Creating backup for {} at {}",
            src_db.display(),
            bak_db.display()
        );
        std::fs::copy(src_db, bak_db)?;
        log::trace!(
            "Creating backup for {} at {}",
            src_jrn.display(),
            bak_jrn.display()
        );
        std::fs::copy(src_jrn, bak_jrn)?;

        if should_open {
            self.open().await?;
        }

        Ok(backup_id)
    }

    /// List all backups available in backup folder
    pub async fn list_backups(&self) -> Result<Vec<uuid::Uuid>> {
        let backup_folder = self
            .connection_kind
            .get_backup_folder()
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;

        let total_files = std::fs::read_dir(&backup_folder)?.count();
        let mut result = Vec::<uuid::Uuid>::with_capacity(total_files);

        for entry in std::fs::read_dir(backup_folder)? {
            let path = entry?.path();
            if Some(OsStr::new("BAK")) == path.extension() {
                if let Some(file_name) = path.file_name() {
                    if let Some(id) = file_name.to_string_lossy().split("__").last() {
                        if let Ok(uuid) = uuid::Uuid::parse_str(id) {
                            result.push(uuid);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Restores current database to a previous backup
    pub async fn restore_backup(&mut self, backup_id: &uuid::Uuid) -> Result<()> {
        let should_open = self.is_open();
        self.close().await?;

        let id_str = SqliteDbHandler::get_backup_id_string(backup_id);

        let src_db = self
            .connection_kind
            .get_db_path()
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;
        let bak_db = self
            .connection_kind
            .get_backup_db_path(&id_str)
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;

        let src_jrn = self
            .connection_kind
            .get_journal_path()
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;
        let bak_jrn = self
            .connection_kind
            .get_backup_journal_path(&id_str)
            .ok_or(DatabaseHandlerError::PathNotAvailable)?;

        std::fs::copy(bak_db, src_db)?;
        std::fs::copy(bak_jrn, src_jrn)?;

        if should_open {
            self.open().await?;
        }

        Ok(())
    }

    fn get_backup_id_string(id: &uuid::Uuid) -> String {
        id.to_string().replace("-", "")
    }
}
