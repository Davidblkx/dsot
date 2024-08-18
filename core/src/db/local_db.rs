use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Release, Acquire};
use std::time::Duration;

use diesel::sqlite::SqliteConnection;

use crate::error::Result;

pub struct LocalDB {
    path: String,
    locked: AtomicBool,
}

pub struct ConnectionGuard<'a> {
    db: &'a LocalDB,
    connection: SqliteConnection,
}

impl LocalDB {
    pub (crate) fn new(path: &str) -> Self {
        LocalDB {
            path: path.to_string(),
            locked: AtomicBool::new(false)
        }
    }

    pub fn lock<'a>(&'a self) -> Result<ConnectionGuard<'a>> {
        let mut count = 0;
        while self.locked.swap(true, Acquire) {
            count += 1;
            if count < 1000 {
                std::hint::spin_loop();
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        }

        match super::connect_db(&self.path) {
            Ok(conn) => {
                return Ok(ConnectionGuard {
                    db: self,
                    connection: conn
                });
            },
            Err(e) => {
                self.locked.store(false, Release);
                return Err(e);
            }
        }
    }
}

impl<'a> ConnectionGuard<'a> {
    pub fn get_conn(&'a self) -> &'a SqliteConnection {
        &self.connection
    }
}

impl Drop for ConnectionGuard<'_> {
    fn drop(&mut self) {
        self.db.locked.store(false, Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_multithread() {
        let path = "is_sync.db".to_string();
        let db = LocalDB::new(&path);

        std::thread::scope(|s| {
            s.spawn(|| {
                let _g = db.lock().unwrap();
                std::thread::sleep(Duration::from_millis(10));
            });
            s.spawn(|| {
                let _g = db.lock().unwrap();
                std::thread::sleep(Duration::from_millis(10));
            });
        });

        let g = db.lock().unwrap();
        let _c = g.get_conn();
        drop(g);
        std::fs::remove_file(path).unwrap();
    }
}
