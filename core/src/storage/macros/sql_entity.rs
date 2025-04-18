#[macro_export]
macro_rules! dsot_sql_entity {
    ([$table_name:expr] $entity:ident with $update:ident {
        $($prop:ident$(: $column:ident)?),*
    }) => {
        impl $crate::storage::sql::SqlEntity for $entity {
            type Value = $entity;
            type Operation = $update;

            fn get_sql_insert_statement() -> &'static str {
                concat!(
                    "INSERT INTO ",
                    $table_name,
                    " (id",
                    $(
                        ", ",
                        $crate::mu_stringify_last!($prop$(, $column)?),
                    )*
                    ") VALUES (?",
                    $($crate::macro_util_value_or!($prop, ", ?")),*,
                    ")"
                )
            }

            fn get_sql_fetch_by_id_statement() -> &'static str {
                concat!(
                    "SELECT id",
                    $(
                        ", ",
                        $crate::mu_stringify_last!($prop$(, $column)?),
                    )*
                    " FROM ",
                    $table_name,
                    " WHERE id = ?"
                )
            }

            async fn execute_sql_insert(
                mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
                entity: &Self::Value
            ) -> $crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
                sqlx::query::<sqlx::Sqlite>(
                    Self::get_sql_insert_statement()
                )
                .bind(&entity.id)
                $(
                    .bind(&entity.$prop)
                )*
                .execute(&mut *trx)
                .await?;

                Ok(trx)
            }

            async fn execute_sql_fetch_by_id(
                mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
                id: &uuid::Uuid
            ) -> $crate::error::Result<Option<Self::Value>> {
                let result = sqlx::query_as::<sqlx::Sqlite, Self::Value>(
                    Self::get_sql_fetch_by_id_statement()
                )
                .bind(id)
                .fetch_optional(&mut *trx)
                .await?;

                Ok(result)
            }
        }
    };
}
