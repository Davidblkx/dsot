#[macro_export]
macro_rules! dsot_sql_entity {
    (
        $name:ident$([$table_name:expr])? {
            $(
                $field:ident$([$column:expr])?: $field_type:ty $( => $column_type:ident)? $( => Optional($column_type_optional:ident))?,
            )*
        }
    ) => {
        paste::paste! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
            pub struct $name {
                $(
                    pub $field: $field_type,
                )*
            }

            impl crate::db::sql::SqlEntity for $name {
                fn table_name() -> &'static str {
                    crate::stringify_or!([< $name:lower >] $(, $table_name)?)
                }

                fn columns() -> Vec<&'static str> {
                    vec![
                        $(
                            crate::stringify_or!($field $(, $column)?),
                        )*
                    ]
                }

                fn values(&self) -> Vec<String> {
                    vec![
                        $(
                            $(crate::db::sql::SqlValue::[<$column_type:lower>](&self.$field),)?
                            $(match &self.$field {
                                Some(value) => crate::db::sql::SqlValue::[<$column_type_optional:lower>](value),
                                None => crate::db::sql::SqlValue::null(),
                            },)?
                        )*
                    ]
                }
            }
        }
    };
}
