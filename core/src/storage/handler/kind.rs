use std::path::PathBuf;

use crate::error::Result;

pub static SQLITE_DB_EXTENSION: &'static str = "db";
pub static SQLITE_JOURNAL_EXTENSION: &'static str = "db.journal";

/// Type of connection to use
///
/// Also contains logic to build paths and to create any needed folder
pub enum HandlerConnectionKind {
    /// Connects to a database in memory, that only last the session
    Memory,
    /// Connects to a file, creates a new one if required
    File {
        /// Name of database file without extension
        name: String,
        /// Path of folder where file is saved
        data_folder: PathBuf,
        /// Alternative backup folder, if None, <data_folder> is used
        backup_folder: Option<PathBuf>,
    },
}

impl HandlerConnectionKind {
    pub fn new_file<S: Into<String>, P: Into<PathBuf>>(name: S, data_folder: P) -> Self {
        Self::File {
            name: name.into(),
            data_folder: data_folder.into(),
            backup_folder: None,
        }
    }

    pub fn with_backup_folder<P: Into<PathBuf>>(self, folder: P) -> Self {
        match self {
            Self::Memory => self,
            Self::File {
                name,
                data_folder,
                backup_folder: _,
            } => Self::File {
                name,
                data_folder,
                backup_folder: Some(folder.into()),
            },
        }
    }

    pub fn get_db_path(&self) -> Option<PathBuf> {
        match self {
            Self::Memory => None,
            Self::File {
                name,
                data_folder,
                backup_folder: _,
            } => Some(
                data_folder
                    .as_path()
                    .join(name)
                    .with_extension(SQLITE_DB_EXTENSION),
            ),
        }
    }

    pub fn get_journal_path(&self) -> Option<PathBuf> {
        match self {
            Self::Memory => None,
            Self::File {
                name,
                data_folder,
                backup_folder: _,
            } => Some(
                data_folder
                    .as_path()
                    .join(name)
                    .with_extension(SQLITE_JOURNAL_EXTENSION),
            ),
        }
    }

    pub fn get_backup_db_path(&self, backup_id: &str) -> Option<PathBuf> {
        match self {
            Self::Memory => None,
            Self::File {
                name,
                data_folder,
                backup_folder,
            } => Some(
                backup_folder
                    .as_ref()
                    .unwrap_or(data_folder)
                    .as_path()
                    .join(name)
                    .with_extension(format!("{}__{}.BAK", SQLITE_DB_EXTENSION, backup_id)),
            ),
        }
    }

    pub fn get_backup_journal_path(&self, backup_id: &str) -> Option<PathBuf> {
        match self {
            Self::Memory => None,
            Self::File {
                name,
                data_folder,
                backup_folder,
            } => Some(
                backup_folder
                    .as_ref()
                    .unwrap_or(data_folder)
                    .as_path()
                    .join(name)
                    .with_extension(format!("{}__{}.BAK", SQLITE_JOURNAL_EXTENSION, backup_id)),
            ),
        }
    }

    pub fn get_data_folder(&self) -> Option<&PathBuf> {
        match self {
            Self::Memory => None,
            Self::File {
                name: _,
                data_folder,
                backup_folder: _,
            } => Some(data_folder),
        }
    }

    pub fn get_backup_folder(&self) -> Option<&PathBuf> {
        match self {
            Self::Memory => None,
            Self::File {
                name: _,
                data_folder,
                backup_folder,
            } => Some(backup_folder.as_ref().unwrap_or(data_folder)),
        }
    }

    /// Check if folders are created, and create if needed
    pub fn ensure_folders(&self) -> Result<()> {
        if let Some(path) = self.get_data_folder() {
            if !path.exists() {
                log::trace!("Creating data folder: {}", path.display());
                std::fs::create_dir_all(path)?;
            }
        }

        if let Some(path) = self.get_backup_folder() {
            if !path.exists() {
                log::trace!("Creating backup folder: {}", path.display());
                std::fs::create_dir_all(path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_file_works_as_expected() {
        let h = HandlerConnectionKind::new_file("my_db", "/data/dir");
        if let HandlerConnectionKind::File {
            name,
            data_folder,
            backup_folder,
        } = h
        {
            assert_eq!("my_db", name);
            assert_eq!(PathBuf::from("/data/dir"), data_folder);
            assert!(backup_folder.is_none())
        } else {
            panic!("new_file should match File branch")
        }

        let h = HandlerConnectionKind::new_file("my_db", "/data/dir")
            .with_backup_folder("/data/backup");
        if let HandlerConnectionKind::File {
            name,
            data_folder,
            backup_folder,
        } = h
        {
            assert_eq!("my_db", name);
            assert_eq!(PathBuf::from("/data/dir"), data_folder);
            assert_eq!(PathBuf::from("/data/backup"), backup_folder.unwrap());
        } else {
            panic!("new_file should match File branch")
        }

        let h = HandlerConnectionKind::new_file("my_db".to_string(), PathBuf::from("/data/dir"))
            .with_backup_folder(PathBuf::from("/data/backup"));
        if let HandlerConnectionKind::File {
            name,
            data_folder,
            backup_folder,
        } = h
        {
            assert_eq!("my_db", name);
            assert_eq!(PathBuf::from("/data/dir"), data_folder);
            assert_eq!(PathBuf::from("/data/backup"), backup_folder.unwrap());
        } else {
            panic!("new_file should match File branch")
        }
    }

    #[test]
    pub fn get_db_path_returns_expected_value() {
        let result = HandlerConnectionKind::new_file("my_db", "/data/dir")
            .get_db_path()
            .unwrap();
        assert_eq!(PathBuf::from("/data/dir/my_db.db"), result);
        assert!(HandlerConnectionKind::Memory.get_db_path().is_none());
    }

    #[test]
    pub fn get_journal_path_returns_expected_value() {
        let result = HandlerConnectionKind::new_file("my_db", "/data/dir")
            .get_journal_path()
            .unwrap();
        assert_eq!(PathBuf::from("/data/dir/my_db.db.journal"), result);
        assert!(HandlerConnectionKind::Memory.get_db_path().is_none());
    }

    #[test]
    pub fn get_backup_db_path_returns_expected_value() {
        assert!(
            HandlerConnectionKind::Memory
                .get_backup_db_path("id")
                .is_none()
        );
        let result = HandlerConnectionKind::new_file("my_db", "/data/dir")
            .get_backup_db_path("id")
            .unwrap();
        assert_eq!(PathBuf::from("/data/dir/my_db.db__id.BAK"), result);
        let result = HandlerConnectionKind::new_file("my_db", "/data/dir")
            .with_backup_folder("/data/backup")
            .get_backup_db_path("id")
            .unwrap();
        assert_eq!(PathBuf::from("/data/backup/my_db.db__id.BAK"), result);
    }

    #[test]
    pub fn get_backup_journal_path_returns_expected_value() {
        assert!(
            HandlerConnectionKind::Memory
                .get_backup_journal_path("id")
                .is_none()
        );
        let result = HandlerConnectionKind::new_file("my_db", "/data/dir")
            .get_backup_journal_path("id")
            .unwrap();
        assert_eq!(PathBuf::from("/data/dir/my_db.db.journal__id.BAK"), result);
        let result = HandlerConnectionKind::new_file("my_db", "/data/dir")
            .with_backup_folder("/data/backup")
            .get_backup_journal_path("id")
            .unwrap();
        assert_eq!(
            PathBuf::from("/data/backup/my_db.db.journal__id.BAK"),
            result
        );
    }
}
