use crate::storage::sql::{SqlOperation, SqlOperationHandler, SqlResult, SqlTransaction};

macro_rules! entity_enum {
    ($($n:literal: $name:ident),*) => {
        pub enum DsotEntity {
            $($name),*
        }

        impl DsotEntity {
            pub fn from_id(id: u32) -> Option<Self> {
                match id {
                    $($n => Some(DsotEntity::$name),)*
                    _ => None,
                }
            }

            pub fn get_id(&self) -> u32 {
                match self {
                    $(DsotEntity::$name => $n,)*
                }
            }
        }

        paste::paste! {
            impl SqlOperationHandler for DsotEntity {
                async fn apply_sql_op(trx: SqlTransaction, op: &SqlOperation) -> SqlResult<()> {
                    match Self::from_id(op.get_entity()) {
                        $(Some(DsotEntity::$name) => super::entities::[<$name:snake>]::sql::[<$name Sql>]::execute_operation(trx, op).await,)*
                        _ => todo!(),
                    }
                }
            }
        }
    };
}

entity_enum! {
    1: Artist,
    2: ArtistAliases,
    3: Album,
    4: AlbumArtist,
    5: Inbox,
    6: MusicFile,
    7: Recording,
    8: Release,
    9: ReleaseMedia,
    10: Storage,
    11: Track,
    12: Work,
    13: ArtistAlias
}
