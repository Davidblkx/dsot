pub trait SqlEntity {
    type Value;
    type Operation;

    fn execute_sql_create(
        trx: sqlx::Transaction<'static, sqlx::Sqlite>,
        entity: &Self::Value
    ) -> impl Future<Output = crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>>>;
}
