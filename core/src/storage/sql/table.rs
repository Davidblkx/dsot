pub trait SqlTable {
    type Entity;
    type UpdateOp;

    fn table() -> &'static str;

    fn execute_create(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        entity: &Self::Entity
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;

    fn execute_update(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid,
        op: &Self::UpdateOp
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;

    fn execute_delete(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;

    fn execute_fetch(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid
    ) -> impl Future<Output = crate::error::Result<(sqlx::Transaction<'static, sqlx::Sqlite>, Self::Entity)>>;
}
