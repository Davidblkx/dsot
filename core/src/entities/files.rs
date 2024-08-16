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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file_with_path() {
        let f = File::new("test.txt");

        assert_eq!(f.path, "test.txt");
        assert_eq!(f.get_path(), Path::new("test.txt"));
    }

    #[test]
    fn has_uuid() {
        let f = File::new("test.txt");

        f.get_id_uuid().expect("Failed to get UUID");
    }

    #[test]
    fn move_file() {
        let mut f = File::new("test.txt");


        std::fs::File::create("test.txt").expect("Failed to create file");
        f.move_to("test2.txt").expect("Failed to move file");

        assert_eq!(f.path, "test2.txt");
        assert_eq!(f.get_path(), Path::new("test2.txt"));
        assert!(!Path::new("test.txt").exists());

        std::fs::remove_file("test2.txt").expect("Failed to remove file");
    }

    #[test]
    fn copy_file() {
        let f = File::new("test_copy.txt");

        std::fs::File::create("test_copy.txt").expect("Failed to create file");
        let f2 = f.copy_to("test_copy2.txt").expect("Failed to copy file");

        assert_eq!(f2.path, "test_copy2.txt");
        assert_eq!(f2.get_path(), Path::new("test_copy2.txt"));
        assert!(Path::new("test_copy.txt").exists());
        assert!(Path::new("test_copy2.txt").exists());

        std::fs::remove_file("test_copy.txt").expect("Failed to remove file");
        std::fs::remove_file("test_copy2.txt").expect("Failed to remove file");
    }
}
