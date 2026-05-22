use crate::{SyncEntity, dser::MessagePackError};
use thiserror::Error;
use uuid::Uuid;

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
    type RepoEntity: SyncEntity;

    fn get_table_name() -> &'static str;

    async fn insert<'a, E>(executor: E, entity: &Self::RepoEntity) -> Result<()>
    where
        E: ::sqlx::prelude::Executor<'a, Database = ::sqlx::Sqlite>;

    async fn delete<'a, E>(executor: E, id: ::uuid::Uuid) -> Result<()>
    where
        E: ::sqlx::prelude::Executor<'a, Database = ::sqlx::Sqlite>;

    async fn restore<'a, E>(executor: E, id: ::uuid::Uuid) -> Result<()>
    where
        E: ::sqlx::prelude::Executor<'a, Database = ::sqlx::Sqlite>;

    async fn update<'a, E>(
        executor: E,
        id: ::uuid::Uuid,
        updates: Vec<crate::model::UpdateColumnOp>,
    ) -> Result<()>
    where
        E: ::sqlx::prelude::Executor<'a, Database = ::sqlx::Sqlite>;

    async fn get<'a, E>(executor: E, id: ::uuid::Uuid) -> Result<Self::RepoEntity>
    where
        E: ::sqlx::prelude::Executor<'a, Database = ::sqlx::Sqlite>;

    async fn list<'a, E>(executor: E, query: ListQuery) -> Result<Vec<Self::RepoEntity>>
    where
        E: ::sqlx::prelude::Executor<'a, Database = ::sqlx::Sqlite>;

    async fn exec_op<'a, E>(executor: E, op: crate::model::SyncOperation) -> Result<()>
    where
        E: ::sqlx::prelude::Executor<'a, Database = ::sqlx::Sqlite>;
}
