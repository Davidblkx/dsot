use thiserror::Error;
use uuid::Uuid;

use crate::{SyncEntity, dser::MessagePackError};

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("entity [{0}] not found: {1}")]
    EntityNotFound(&'static str, Uuid),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    SerializationError(#[from] MessagePackError),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug, Clone, Copy)]
pub struct ListQuery {
    pub count: i64,
    pub offset: i64,
}

pub trait SyncEntityRepository {
    type RepoEntity: SyncEntity<Entity = Self::RepoEntity>;

    fn get_table_name() -> &'static str;

    async fn insert(
        executor: &mut ::sqlx::SqliteConnection,
        entity: &Self::RepoEntity,
    ) -> Result<()>;

    async fn delete(executor: &mut ::sqlx::SqliteConnection, id: ::uuid::Uuid) -> Result<()>;

    async fn restore(executor: &mut ::sqlx::SqliteConnection, id: ::uuid::Uuid) -> Result<()>;

    async fn update(
        executor: &mut ::sqlx::SqliteConnection,
        id: ::uuid::Uuid,
        updates: Vec<crate::model::UpdateColumnOp>,
    ) -> Result<()>;

    async fn get(
        executor: &mut ::sqlx::SqliteConnection,
        id: ::uuid::Uuid,
    ) -> Result<Self::RepoEntity>;

    async fn try_get(
        executor: &mut ::sqlx::SqliteConnection,
        id: ::uuid::Uuid,
    ) -> Result<Option<Self::RepoEntity>>;

    async fn list(
        executor: &mut ::sqlx::SqliteConnection,
        query: ListQuery,
    ) -> Result<Vec<Self::RepoEntity>>;

    async fn exec_op(
        executor: &mut ::sqlx::SqliteConnection,
        op: crate::model::SyncOperation,
    ) -> Result<()>;

    async fn search(
        executor: &mut ::sqlx::SqliteConnection,
        query: String,
    ) -> Result<Vec<Self::RepoEntity>>;
}
