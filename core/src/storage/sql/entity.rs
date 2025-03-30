pub trait SqlEntity {
    type Value;
    type Operation;

    fn get_sql_insert_statement() -> &'static str;

    fn execute_sql_insert(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        entity: &Self::Value
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;
}
