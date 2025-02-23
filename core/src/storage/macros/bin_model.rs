#[macro_export]
macro_rules! dsot_storage_declare_model_de_v0 {
    ($name: ident $(, $rest: ident)*) => {
        impl $crate::storage::BinModel for $name {
            type Model = $name;

            fn deserialize<'a>(data: &'a [u8]) -> $crate::error::Result<Self::Model> {
                let data = $crate::storage::parse_data_with_version(data)?;
                Self::Model::deserialize_version(data.data, data.version)
            }

            fn deserialize_version<'a>(data: &'a [u8], version: u32) -> $crate::error::Result<Self::Model> {
                if version != 0 {
                    return Err($crate::error::DsotError::DataFormatError(format!("Entity {} => Invalid version: {}", stringify!($name), version)));
                }

                $crate::storage::deserialize_bincode1(data)
                    .map_err(|e| $crate::error::DsotError::DeserializationError(e))
            }

            fn serialize(&self) -> $crate::error::Result<Vec<u8>> {
                let data: Vec<u8> = $crate::storage::serialize_bincode1(self)
                    .map_err(|e| $crate::error::DsotError::SerializationError(e))?;

                $crate::storage::serialize_data_with_version(0, &data)
            }

            fn need_update<'a>(data: &'a [u8]) -> $crate::error::Result<bool> {
                let data = $crate::storage::parse_data_with_version(data)?;
                Ok(data.version != 0)
            }
        }
    };
}

#[macro_export]
macro_rules! dsot_storage_declare_model_de {
    ($minor:ident, $major:ident, $version: literal) => {
        impl $crate::storage::BinModel for $major {
            type Model = $major;

            fn deserialize<'a>(data: &'a [u8]) -> $crate::error::Result<Self::Model> {
                let data = $crate::storage::parse_data_with_version(data)?;
                Self::Model::deserialize_version(data.data, data.version)
            }

            fn deserialize_version<'a>(data: &'a [u8], version: u32) -> $crate::error::Result<Self::Model> {
                match version {
                    v if v < $version => {
                        let model = $minor::deserialize_version(data, version)?;
                        Ok(model.into())
                    },
                    v if v == $version => {
                        $crate::storage::deserialize_bincode1(data)
                            .map_err(|e| $crate::error::DsotError::DeserializationError(e))
                    },
                    _ => Err($crate::error::DsotError::DataFormatError(format!("Entity {} => Invalid version: {}", stringify!($major), version))),
                }
            }

            fn serialize(&self) -> $crate::error::Result<Vec<u8>> {
                let data: Vec<u8> = $crate::storage::serialize_bincode1(self)
                    .map_err(|e| $crate::error::DsotError::SerializationError(e))?;

                $crate::storage::serialize_data_with_version($version, &data)
            }

            fn need_update<'a>(data: &'a [u8]) -> $crate::error::Result<bool> {
                let data = $crate::storage::parse_data_with_version(data)?;
                Ok(data.version != $version)
            }
        }
    };
}

#[macro_export]
macro_rules! dsot_storage_declare_all_model_de {
    ($version: literal: $name: ident) => {};
    ($version_prev: literal: $name_prev: ident, $version: literal: $name: ident $(, $v: literal: $n:ident)*) => {
        $crate::dsot_storage_declare_model_de!($name_prev, $name, $version);

        $crate::dsot_storage_declare_all_model_de!($version: $name $(, $v: $n)*);
    };
}

#[macro_export]
macro_rules! dsot_storage_declare_model_type {
    ($name: ident, $type: ident, $($rest: ident),*) => {
        $crate::dsot_storage_declare_model_type!($name, $($rest),*);
    };
    ($name: ident, $type: ident) => {
        pub type $name = $type;
    };
}

/// Declares a new model that can be stored in the storage
///
/// The model must implement the serde::Serialize and serde::Deserialize traits
///
/// Example:
///
/// ```
/// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// pub struct ModelV0 {
///    a: u32,
///    b: u32,
/// }
///
/// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// pub struct ModelV1 {
///   a: u32,
///   b: u32,
///   c: u32,
/// }
///
/// impl From<ModelV0> for ModelV1 {
///    fn from(model: ModelV0) -> Self {
///       Self {
///         a: model.a,
///         b: model.b,
///         c: 0,
///       }
///    }
/// }
///
/// dsot_storage_declare_model!(Model {
///   0: ModelV0,
///   1: ModelV1
/// });
/// ```
///
/// This will create a new model called Model that can be stored in the storage
///
/// The model will have two versions, version 0 and version 1
///
/// The model will have a alias called Model that will be the latest version of the model
#[macro_export]
macro_rules! dsot_storage_declare_model {
    ($name: ident {
        $($version: literal: $type: ident),*
    }) => {
        $crate::dsot_storage_declare_model_type!($name, $($type),*);

        $crate::dsot_storage_declare_model_de_v0!($($type),*);

        $crate::dsot_storage_declare_all_model_de!($($version: $type),*);
    };
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct ModelV0 {
        a: u32,
        b: u32,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct ModelV1 {
        a: u32,
        b: u32,
        c: u32,
    }

    impl From<ModelV0> for ModelV1 {
        fn from(model: ModelV0) -> Self {
            Self {
                a: model.a,
                b: model.b,
                c: 0,
            }
        }
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
    pub struct ModelV2 {
        a: u32,
        b: u32,
        c: u32,
        d: String
    }

    impl From<ModelV1> for ModelV2 {
        fn from(model: ModelV1) -> Self {
            Self {
                a: model.a,
                b: model.b,
                c: model.c,
                d: "".to_string(),
            }
        }
    }

    dsot_storage_declare_model!(Model {
        0: ModelV0,
        1: ModelV1,
        2: ModelV2
    });

    #[test]
    fn test_can_serialize_model() {
        use crate::storage::BinModel;

        let model = Model {
            a: 1,
            b: 2,
            c: 3,
            d: "test".to_string(),
        };

        let data = model.serialize().unwrap();
        let same_model = Model::deserialize(&data).unwrap();

        assert_eq!(model, same_model);
    }

    #[test]
    fn test_can_deserialize_older_model() {
        use crate::storage::BinModel;

        let model = ModelV0 {
            a: 1,
            b: 2
        };

        let data = model.serialize().unwrap();
        let new_model = Model::deserialize(&data).unwrap();

        assert_eq!(new_model, Model {
            a: 1,
            b: 2,
            c: 0,
            d: "".to_string(),
        });
    }

    #[test]
    fn test_check_need_update() {
        use crate::storage::BinModel;

        let model = ModelV0 {
            a: 1,
            b: 2
        };
        let older_data = model.serialize().unwrap();

        let model = ModelV2 {
            a: 1,
            b: 2,
            c: 3,
            d: "test".to_string()
        };
        let newer_data = model.serialize().unwrap();

        assert!(Model::need_update(&older_data).unwrap());
        assert!(!Model::need_update(&newer_data).unwrap());
    }

}
