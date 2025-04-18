#[macro_export]
macro_rules! dsot_sql_entity {
    ([$table_name:expr] $entity:ident with $update:ident {
        $($prop:ident$(: $column:ident)?),*
    }) => {

        impl $entity {
            paste::paste! {
                $(
                    pub fn [< get_sql_update_ $prop >]() -> &'static str {
                        concat!(
                            "UPDATE ",
                            $table_name,
                            " SET ",
                            $crate::mu_stringify_last!($prop$(, $column)?),
                            " = ? WHERE id = ?"
                        )
                    }
                )*
            }
        }

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

            fn get_delete_sql_statement() -> &'static str {
                concat!(
                    "DELETE FROM ",
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
            ) -> $crate::error::Result<(sqlx::Transaction<'static, sqlx::Sqlite>, Option<Self::Value>)> {
                let result = sqlx::query_as::<sqlx::Sqlite, Self::Value>(
                    Self::get_sql_fetch_by_id_statement()
                )
                .bind(id)
                .fetch_optional(&mut *trx)
                .await?;

                Ok((trx, result))
            }

            async fn execute_sql_delete(
                mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
                id: &uuid::Uuid
            ) -> $crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
                sqlx::query::<sqlx::Sqlite>(
                    Self::get_delete_sql_statement()
                )
                .bind(id)
                .execute(&mut *trx)
                .await?;

                Ok(trx)
            }

            async fn execute_sql_update(
                mut trx: sqlx::Transaction<'static, sqlx::Sqlite>,
                id: &uuid::Uuid,
                op: &Self::Operation
            ) -> $crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
                paste::paste! {
                    match op {
                        $(
                            Self::Operation::[<Set $prop:camel>](value) => {
                                sqlx::query::<sqlx::Sqlite>(
                                    Self::Value::[<get_sql_update_ $prop>]()
                                )
                                .bind(value)
                                .bind(id)
                                .execute(&mut *trx)
                                .await?;

                                Ok(trx)
                            }
                        )*
                    }
                }
            }
        }

        impl $crate::storage::SqlOperationHandler for $entity {
            async fn apply_sql_op(
                trx: sqlx::Transaction<'static, sqlx::Sqlite>,
                op: &crate::storage::SqlOperation,
            ) -> $crate::error::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
                match op {
                    crate::storage::SqlOperation::Create { data, .. } => {
                        let entity = $entity::deserialize(data)?;
                        $entity::execute_sql_insert(trx, &entity).await
                    }
                    crate::storage::SqlOperation::Update { id, action, .. } => {
                        let op = $update::deserialize(action)?;
                        $entity::execute_sql_update(trx, id, &op).await
                    }
                    crate::storage::SqlOperation::Delete { id, .. } => {
                        $entity::execute_sql_delete(trx, id).await
                    }
                }
            }
        }
    };
}
