pub type RepositoryError = crate::DBSyncError;
pub type Result<T> = std::result::Result<T, crate::DBSyncError>;

use crate::SyncEntity;

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
