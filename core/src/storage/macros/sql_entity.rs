#[macro_export]
macro_rules! dsot_sql_entity {
    ([$table_name:expr] $entity:ident with $update:ident {
        $($prop:ident$([ $column:ident])?$(: $prop_type:ty)?),*
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

                pub fn get_list_sql_statement() -> &'static str {
                    concat!(
                        "SELECT * FROM ",
                        $table_name,
                        " ORDER BY id",
                        " LIMIT ? OFFSET ? "
                    )
                }

                pub fn get_search_sql_statement() -> &'static str {
                    concat!(
                        "SELECT t.* FROM ",
                        $table_name,
                        " t",
                        " JOIN ",
                        $table_name,
                        "_fts fts",
                        " ON t.id = fts.id",
                        " WHERE ",
                        $table_name,
                        "_fts MATCH ?",
                        " ORDER BY rank",
                        " LIMIT ? OFFSET ? "
                    )
                }

                pub fn get_count_sql_statement() -> &'static str {
                    concat!(
                        "SELECT COUNT(id) FROM ",
                        $table_name
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
                                log::trace!("Updating {}.{} for id {} with {:?}", $table_name, stringify!($prop), id, value);

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

                pub async fn list(mut trx: SqlTransaction, length: i64, skip: i64) -> SqlResult<Vec<$entity>> {
                    let res = sqlx::query_as::<sqlx::Sqlite, $entity>(
                        Self::get_list_sql_statement()
                    )
                    .bind(length)
                    .bind(skip)
                    .fetch_all(&mut *trx)
                    .await?;

                    Ok((trx, res))
                }

                pub async fn search(mut trx: SqlTransaction, search: &str, length: i64, skip: i64) -> SqlResult<Vec<$entity>> {
                    let res = sqlx::query_as::<sqlx::Sqlite, $entity>(
                        Self::get_search_sql_statement()
                    )
                    .bind(search)
                    .bind(length)
                    .bind(skip)
                    .fetch_all(&mut *trx)
                    .await?;

                    Ok((trx, res))
                }

                pub async fn count(mut trx: SqlTransaction) -> SqlResult<i64> {
                    let count: i64 = sqlx::query_scalar(
                        Self::get_count_sql_statement()
                    )
                    .fetch_one(&mut *trx)
                    .await?;

                    Ok((trx, count))
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

            pub struct [< $entity SqlOperation>]<'a> {
                entity: &'a $entity,
            }

            impl<'a> [< $entity SqlOperation>]<'a> {
                pub fn new(entity: &'a $entity) -> Self {
                    Self { entity }
                }

                /// Creates a SQL operation to insert this entity into the database.
                pub fn create(&self) -> $crate::error::Result<$crate::storage::SqlOperation> {
                    let op = crate::storage::SqlOperation::Create {
                        id: self.entity.id,
                        entity: $entity::get_dsot_entity_id(),
                        data: self.entity.serialize()?,
                    };
                    Ok(op)
                }

                /// Creates a SQL operation to delete this entity in the database.
                pub fn delete(&self) -> $crate::storage::SqlOperation {
                    crate::storage::SqlOperation::Delete {
                        id: self.entity.id,
                        entity: $entity::get_dsot_entity_id(),
                    }
                }

                /// Creates a SQL operation to update this entity in the database.
                pub fn update(&self, op: $update) -> $crate::error::Result<$crate::storage::SqlOperation> {
                    let op = crate::storage::SqlOperation::Update {
                        id: self.entity.id,
                        entity: $entity::get_dsot_entity_id(),
                        action: op.serialize()?,
                    };
                    Ok(op)
                }

                $(
                    $(
                        pub fn [< update_ $prop >](&self, value: $prop_type) -> $crate::error::Result<$crate::storage::SqlOperation> {
                            self.update($update::[<Set $prop:camel>](value))
                        }
                    )?
                )*
            }

            impl $entity {
                /// Returns the internal id of the SQL table for this entity.
                pub fn get_dsot_entity_id() -> u32 {
                    $crate::model::DsotEntity::[< $entity:camel >].get_id()
                }

                /// Returns the SQL operation handler for this entity.
                pub fn sql_operation<'a>(&'a self) -> [< $entity SqlOperation>]<'a> {
                    [< $entity SqlOperation>]::new(self)
                }
            }
        }
    };
}
