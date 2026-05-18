use crate::model::SyncOperation;
use sqlx::{Executor, sqlite::Sqlite};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("entity [{0}] not found: {1}")]
    EntityNotFound(&'static str, Uuid),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug, Clone, Copy)]
pub struct ListQuery {
    pub count: u64,
    pub offset: u64,
}

pub trait SyncEntityRepository {
    type Entity;

    /// Returns the name of the table for this entity.
    fn get_table_name() -> &'static str;

    /// Returns the entity with the given ID.
    async fn get<'a, E>(executor: E, id: &Uuid) -> Result<Self::Entity>
    where
        E: Executor<'a, Database = Sqlite>;

    /// Inserts the given entity into the database.
    async fn insert<'a, E>(executor: E, value: &Self::Entity) -> Result<()>
    where
        E: Executor<'a, Database = Sqlite>;

    /// Updates the given entity in the database.
    async fn update<'a, E>(executor: E, value: &Self::Entity) -> Result<()>
    where
        E: Executor<'a, Database = Sqlite>;

    /// Soft Delete the entity with the given ID from the database.
    async fn delete<'a, E>(executor: E, id: &Uuid) -> Result<()>
    where
        E: Executor<'a, Database = Sqlite>;

    /// Undelete the entity with the given ID from the database.
    async fn restore<'a, E>(executor: E, id: &Uuid) -> Result<()>
    where
        E: Executor<'a, Database = Sqlite>;

    /// Returns all entities in the database.
    async fn list<'a, E>(executor: E, query: ListQuery) -> Result<Vec<Self::Entity>>
    where
        E: Executor<'a, Database = Sqlite>;

    async fn exec_op<'a, E>(executor: E, op: SyncOperation) -> Result<()>
    where
        E: Executor<'a, Database = Sqlite>;
}
