use dsot_core::model::{
    JournalOperation,
    entities::user::{User, sql::UserSql},
};
use thiserror::Error;
use uuid::Uuid;

use crate::Runtime;
use crate::error::Result;

#[derive(Error, Debug)]
pub enum UsersError {
    #[error("User with name {0}, already exists")]
    DuplicatedUser(String),
}

impl UsersError {
    pub fn to_err<T>(self) -> Result<T> {
        Err(self.into())
    }
}

pub trait Users {
    fn has_user(&self, name: &str) -> impl Future<Output = Result<bool>>;
    fn create_user(&self, name: &str) -> impl Future<Output = Result<Uuid>>;
    fn list_users(&self) -> impl Future<Output = Result<Vec<User>>>;
}

impl Users for Runtime {
    async fn create_user(&self, name: &str) -> Result<Uuid> {
        if self.has_user(name).await? {
            return UsersError::DuplicatedUser(name.to_string()).to_err();
        }

        log::trace!("Prepare to create new user: {}", name);
        let user = User::new(name);
        let user_op = user.sql_operation().create()?;
        log::trace!("Creating database for user: {}", name);
        self.db.get_user_db(name).await?;
        log::trace!("Registering user: {}", name);

        let db = self.db.get_lib_db()?;
        db.create_and_apply(JournalOperation::SQL(user_op)).await?;

        Ok(user.id)
    }

    async fn has_user(&self, name: &str) -> Result<bool> {
        let db = self.db.get_lib_db()?;
        let trx = db.create_db_transaction().await?;

        let (_, user) = UserSql::fetch_by_user_name(trx, name).await?;

        Ok(user.is_some())
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        let db = self.db.get_lib_db()?;
        let trx = db.create_db_transaction().await?;

        let (_, users) = UserSql::fetch_all(trx).await?;

        Ok(users)
    }
}
