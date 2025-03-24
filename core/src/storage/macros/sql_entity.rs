#[macro_export]
macro_rules! dsot_sql_entity {
    ([$table_name:expr] $entity:ident with $update:ident {
        $($prop:ident$(: $column:ident)?),*
    }) => {
        impl $crate::storage::sql::SqlEntity for $entity {
            type Value = $entity;
            type Operation = $update;

            async fn execute_sql_create(
                mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
                entity: &Self::Value
            ) -> $crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
                // TODO: Think of a way to make this work with a macro
                sqlx::query!(
                    format!(
                        "INSERT INTO {} (id, {}) VALUES (?, {})",
                        $table_name,
                        stringify!($(macro_util_value_or!($prop$(, $column)?)),*)
                        stringify!($(macro_util_value_or!($prop, "?")),*)
                    ),
                    entity.id,
                    $(
                        entity.$prop,
                    )*
                )
                .execute(&mut *trx)
                .await?;

                Ok(trx)
            }
        }
    };
}
