#[macro_export]
macro_rules! storage_schema_v0 {
    ($name: ident[$table_name: expr] => $get_id: expr) => {
        impl $crate::storage::Migration for $name {
            type PrevVersion = ();

            fn migrate(entry: &$crate::storage::StorageEntry) -> $crate::error::Result<$crate::storage::MigrationResult> {
                let needs_migration = Self::needs_migration(entry)?;
                if !needs_migration {
                    return Ok($crate::storage::MigrationResult {
                        value: None,
                        version: entry.version,
                    });
                } else {
                    // Only version 0 is supported
                    Err($crate::error::DsotError::DataVersionMismatch)
                }
            }

            fn needs_migration(entry: &$crate::storage::StorageEntry) -> $crate::error::Result<bool> {
                if entry.version == 0 {
                    Ok(false)
                } else {
                    Err($crate::error::DsotError::InvalidStorageVersion { entity: $table_name, version: entry.version })
                }
            }
        }

        impl $crate::storage::StorageSchema for $name {
            type Item = $name;

            fn table_name(&self) -> &'static str {
                $table_name
            }

            fn version(&self) -> u64 {
                0
            }

            fn get_key(&self) -> $crate::error::Result<Vec<u8>> {
                Ok($get_id(self))
            }

            fn to_storage_entry(&self) -> $crate::error::Result<$crate::storage::StorageEntry> {
                let key = self.get_key()?;
                let value = bincode1::serialize(&self).map_err($crate::error::DsotError::SerializationError)?;

                Ok($crate::storage::StorageEntry {
                    key,
                    value,
                    version: self.version(),
                    table: self.table_name(),
                })
            }

            fn from_storage_entry(entry: &$crate::storage::StorageEntry) -> $crate::error::Result<Self::Item> {
                use $crate::storage::Migration;

                let migration_result = Self::Item::migrate(entry)?;

                let data: &Vec<u8> = match &migration_result.value {
                    Some(data) => data,
                    None => &entry.value,
                };

                bincode1::deserialize(data).map_err($crate::error::DsotError::DeserializationError)
            }
        }
    };
}

#[macro_export]
macro_rules! storage_schema {
    ($version:expr => {
        $name: ident[$table_name: expr] => $get_id: expr,
        $from: ident => $get_prev: expr
    }) => {
        impl $crate::storage::Migration for $name {
            type PrevVersion = $from;

            fn migrate(entry: &$crate::storage::StorageEntry) -> $crate::error::Result<$crate::storage::MigrationResult> {
                use $crate::storage::StorageSchema;

                let needs_migration = Self::needs_migration(entry)?;
                if !needs_migration {
                    Ok($crate::storage::MigrationResult {
                        value: None,
                        version: entry.version,
                    })
                } else {
                    match Self::PrevVersion::migrate(entry)?.value {
                        Some(migrated) => {
                            let prev_item: Self::PrevVersion = bincode1::deserialize(&migrated).map_err($crate::error::DsotError::DeserializationError)?;
                            let item = $get_prev(&prev_item).to_storage_entry()?;
                            Ok($crate::storage::MigrationResult {
                                value: Some(item.value),
                                version: $version,
                            })
                        },
                        None => Err($crate::error::DsotError::DataVersionMismatch)
                    }
                }
            }

            fn needs_migration(entry: &$crate::storage::StorageEntry) -> $crate::error::Result<bool> {
                if entry.version == $version {
                    Ok(false)
                } else if entry.version > 0 {
                    Err($crate::error::DsotError::InvalidStorageVersion { entity: $table_name, version: entry.version })
                } else {
                    Ok(true)
                }
            }
        }

        impl $crate::storage::StorageSchema for $name {
            type Item = $name;

            fn table_name(&self) -> &'static str {
                $table_name
            }

            fn version(&self) -> u64 {
                $version
            }

            fn get_key(&self) -> $crate::error::Result<Vec<u8>> {
                Ok($get_id(self))
            }

            fn to_storage_entry(&self) -> $crate::error::Result<$crate::storage::StorageEntry> {
                let key = self.get_key()?;
                let value = bincode1::serialize(&self).map_err($crate::error::DsotError::SerializationError)?;

                Ok($crate::storage::StorageEntry {
                    key,
                    value,
                    version: self.version(),
                    table: self.table_name(),
                })
            }

            fn from_storage_entry(entry: &$crate::storage::StorageEntry) -> $crate::error::Result<Self::Item> {
                use $crate::storage::Migration;

                let migration_result = Self::Item::migrate(entry)?;

                let data: &Vec<u8> = match &migration_result.value {
                    Some(data) => data,
                    None => &entry.value,
                };

                bincode1::deserialize(data).map_err($crate::error::DsotError::DeserializationError)
            }
        }
    };
}
