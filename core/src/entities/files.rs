use std::path::Path;

use diesel::prelude::*;

use crate::error::Result;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::db::schema::files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct File {
    pub id: Vec<u8>,
    pub path: String,
}

impl_uuid_field!(File, id);

impl File {
    pub fn new(path: impl ToString) -> Self {
        Self {
            id: super::utils::new_uuid(),
            path: path.to_string(),
        }
    }

    pub fn get_path<'a>(&'a self) -> &'a Path {
        Path::new(&self.path)
    }

    pub fn move_to(&mut self, new_path: impl ToString) -> Result<()> {
        let n = new_path.to_string();

        std::fs::rename(self.path.as_str(), &n)?;
        self.path = n;
        Ok(())
    }

    pub fn copy_to(&self, new_path: impl ToString) -> Result<File> {
        let n = new_path.to_string();

        std::fs::copy(self.path.as_str(), &n)?;
        Ok(File {
            id: super::utils::new_uuid(),
            path: n,
        })
    }
}
