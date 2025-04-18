pub trait SqlEntity {
    type Value;
    type Operation;

    fn get_sql_insert_statement() -> &'static str;

    fn get_sql_fetch_by_id_statement() -> &'static str;

    fn get_delete_sql_statement() -> &'static str;

    fn execute_sql_insert(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        entity: &Self::Value
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;

    fn execute_sql_fetch_by_id(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid
    ) -> impl Future<Output = crate::error::Result<(sqlx::Transaction<'static, sqlx::Sqlite>, Option<Self::Value>)>>;

    fn execute_sql_delete(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;

    fn execute_sql_update(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid,
        op: &Self::Operation
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;
}
