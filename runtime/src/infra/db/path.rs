use super::error::DbError;
use crate::infra::db::kind::DbKind;
use std::path::PathBuf;

/// Default database name for dsot database.
pub static DB_NAME: &str = "dsot.db";

/// Returns the database filename based on the kind of database.
pub fn get_db_filename<'a>(kind: &DbKind<'a>) -> String {
    match kind {
        DbKind::Main => DB_NAME.to_string(),
        DbKind::User(user_id) => format!("user_{}.db", user_id),
    }
}

/// Builds the path to the database file based on the provided path and kind of database.
pub fn get_db_path<'a>(path: &PathBuf, kind: &DbKind<'a>) -> PathBuf {
    path.join(get_db_filename(kind))
}

/// Creates a new backup path for the database based on the provided path and kind of database.
/// If the backup directory does not exist, it will be created.
/// Path is structured as `<path>/backups/<db_filename>.<timestamp>.BAK`.
/// If `daily_backup` is true, the timestamp will be in the format `YYYYMMDD`, otherwise it will include time as well (`YYYYMMDD_HHMMSS`).
pub fn create_db_backup_path<'a>(
    path: &PathBuf,
    kind: &DbKind<'a>,
    daily_backup: bool,
) -> Result<PathBuf, DbError> {
    let backup_dir = path.join("backups");
    if !backup_dir.exists() {
        std::fs::create_dir_all(&backup_dir)?;
    }

    let db_filename = get_db_filename(kind);
    let timestamp = if daily_backup {
        chrono::Local::now().format("%Y%m%d")
    } else {
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    };
    let backup_file = format!("{}.{}.BAK", db_filename, timestamp);
    let backup_path = backup_dir.join(backup_file);

    if backup_path.exists() {
        return Err(DbError::BackupExists(backup_path.display().to_string()));
    }

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::db::kind::DbKind;

    #[test]
    fn test_get_db_filename() {
        assert_eq!(get_db_filename(&DbKind::Main), DB_NAME);
        assert_eq!(
            get_db_filename(&DbKind::User("test_user")),
            "user_test_user.db"
        );
    }
}
