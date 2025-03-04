use super::entities::*;
use super::SqlEntity;
use super::DbOperation;

use crate::error::Result;
use crate::storage::BinModel;

macro_rules! entity_enum {
    ($($entity:ident = $index:literal),*,) => {
        paste::paste! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub enum DbEntity {
                $(
                    $entity,
                )*
            }

            impl DbEntity {
                pub fn to_id(&self) -> u32 {
                    match self {
                        $(
                            DbEntity::$entity => $index,
                        )*
                    }
                }

                pub fn to_str(&self) -> &'static str {
                    match self {
                        $(
                            DbEntity::$entity => stringify!([<$entity:lower>]),
                        )*
                    }
                }

                pub fn table_name(&self) -> &'static str {
                    match self {
                        $(
                            DbEntity::$entity => $entity::table_name(),
                        )*
                    }
                }

                pub fn columns(&self) -> Vec<&'static str> {
                    match self {
                        $(
                            DbEntity::$entity => $entity::columns(),
                        )*
                    }
                }

                pub fn from_id(id: u32) -> Option<DbEntity> {
                    match id {
                        $(
                            $index => Some(DbEntity::$entity),
                        )*
                        _ => None,
                    }
                }

                pub fn from_str(name: &str) -> Option<DbEntity> {
                    match name {
                        $(
                            stringify!([<$entity:lower>]) => Some(DbEntity::$entity),
                        )*
                        _ => None,
                    }
                }
            }

            impl DbOperation {
                $(
                    pub fn [<create_ $entity:lower>](value: $entity) -> Result<DbOperation> {
                        let data = value.serialize()?;
                        Ok(DbOperation::Create {
                            id: value.id,
                            entity: DbEntity::$entity.to_id(),
                            data,
                        })
                    }

                    pub fn [<update_ $entity:lower>](id: uuid::Uuid, props: Vec<(String, String)>) -> DbOperation {
                        DbOperation::Update {
                            id,
                            entity: DbEntity::$entity.to_id(),
                            props,
                        }
                    }

                    pub fn [<delete_ $entity:lower>](id: uuid::Uuid) -> DbOperation {
                        DbOperation::Delete {
                            id,
                            entity: DbEntity::$entity.to_id(),
                        }
                    }
                )*

                // TODO: Implement logic to generate SQL insert, update and delete operations
            }
        }
    };
}

entity_enum!{
    Artist = 1,
}
