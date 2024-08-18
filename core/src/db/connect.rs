use diesel::Connection;
use crate::error::Result;

pub fn connect_db(db_path: &str) -> Result<diesel::sqlite::SqliteConnection> {
    let conn = diesel::sqlite::SqliteConnection::establish(&db_path)?;
    Ok(conn)
}
