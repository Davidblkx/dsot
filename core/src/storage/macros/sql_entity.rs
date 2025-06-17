#[macro_export]
macro_rules! dsot_sql_entity {
    ([$table_name:expr] $entity:ident with $update:ident {
        $($prop:ident$(: $column:ident)?),*
    }) => {
        use $crate::storage::sql::{SqlTransaction, SqlResult};
        use $crate::storage::BinModel;

        paste::paste! {
            pub struct [< $entity Sql >];

            impl [< $entity Sql >] {
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

                pub fn get_sql_insert_statement() -> &'static str {
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

                pub fn get_sql_fetch_by_id_statement() -> &'static str {
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

                pub fn get_delete_sql_statement() -> &'static str {
                    concat!(
                        "DELETE FROM ",
                        $table_name,
                        " WHERE id = ?"
                    )
                }

                pub async fn insert(
                    mut trx: SqlTransaction,
                    entity: &$entity
                ) -> SqlResult<()> {
                    sqlx::query::<sqlx::Sqlite>(
                        Self::get_sql_insert_statement()
                    )
                    .bind(&entity.id)
                    $(
                        .bind(&entity.$prop)
                    )*
                    .execute(&mut *trx)
                    .await?;

                    Ok((trx, ()))
                }

                pub async fn fetch_by_id(
                    mut trx: SqlTransaction,
                    id: &uuid::Uuid
                ) -> SqlResult<Option<$entity>> {
                    let result = sqlx::query_as::<sqlx::Sqlite, $entity>(
                        Self::get_sql_fetch_by_id_statement()
                    )
                    .bind(id)
                    .fetch_optional(&mut *trx)
                    .await?;

                    Ok((trx, result))
                }

                pub async fn delete(
                    mut trx: SqlTransaction,
                    id: &uuid::Uuid
                ) -> SqlResult<()> {
                    sqlx::query::<sqlx::Sqlite>(
                        Self::get_delete_sql_statement()
                    )
                    .bind(id)
                    .execute(&mut *trx)
                    .await?;

                    Ok((trx, ()))
                }

                pub async fn update(
                    mut trx: SqlTransaction,
                    id: &uuid::Uuid,
                    op: &$update
                ) -> SqlResult<()> {
                    match op {
                        $(
                            $update::[<Set $prop:camel>](value) => {
                                sqlx::query::<sqlx::Sqlite>(
                                    Self::[<get_sql_update_ $prop>]()
                                )
                                .bind(value)
                                .bind(id)
                                .execute(&mut *trx)
                                .await?;

                                Ok((trx, ()))
                            }
                        )*
                    }
                }

                pub async fn execute_operation(
                    trx: SqlTransaction,
                    op: &crate::storage::SqlOperation
                ) -> SqlResult<()> {
                    match op {
                        crate::storage::SqlOperation::Create { data, .. } => {
                            let entity = $entity::deserialize(data)?;
                            Self::insert(trx, &entity).await
                        }
                        crate::storage::SqlOperation::Update { id, action, .. } => {
                            let op = $update::deserialize(action)?;
                            Self::update(trx, id, &op).await
                        }
                        crate::storage::SqlOperation::Delete { id, .. } => {
                            Self::delete(trx, id).await
                        }
                    }
                }
            }

            impl $entity {
                pub fn get_dsot_entity_id() -> u32 {
                    $crate::model::DsotEntity::[< $entity:camel >].get_id()
                }
            }
        }
    };
}
