pub trait SqlEntity {
    type Value;
    type Operation;

    fn get_sql_insert_statement() -> &'static str;

    fn get_sql_fetch_by_id_statement() -> &'static str;

    fn execute_sql_insert(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        entity: &Self::Value
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;

    fn execute_sql_fetch_by_id(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        id: &uuid::Uuid
    ) -> impl Future<Output = crate::error::Result<Option<Self::Value>>>;
}
