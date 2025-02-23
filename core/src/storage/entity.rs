/// Defines the method to get the key for an entity, the key is used to store the entity in the storage
pub trait StorageEntity {
    fn get_storage_key(&self) -> Vec<u8>;

    fn get_storage_name() -> &'static str;
}

/// Defines the method to serialize values to bytes
pub trait HasBytes {
    fn get_bytes(&self) -> &[u8];
}

/// Implements the StorageEntity trait for a model that uses a uuid as the id
#[macro_export]
macro_rules! dsot_storage_use_id_uuid {
    ($name:ident, $storage_name: literal) => {
        use $crate::storage::StorageEntity;

        impl StorageEntity for $name {
            fn get_storage_key(&self) -> Vec<u8> {
                self.id.as_bytes().to_vec()
            }

            fn get_storage_name() -> &'static str {
                $storage_name
            }
        }
    };
}

impl HasBytes for uuid::Uuid {
    fn get_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl HasBytes for String {
    fn get_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl HasBytes for Vec<u8> {
    fn get_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}

impl HasBytes for &[u8] {
    fn get_bytes(&self) -> &[u8] {
        *self
    }
}

impl HasBytes for &str {
    fn get_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
