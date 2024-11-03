macro_rules! declare_model_de_v0 {
    ($name: ident $(, $rest: ident)*) => {
        impl $crate::storage::BinModel for $name {
            type Model = $name;

            fn deserialize<'a>(data: &'a [u8]) -> $crate::error::Result<Self::Model> {
                let data = parse_data(data)?;
                Self::Model::deserialize_version(data.data, data.version)
            }

            fn deserialize_version<'a>(data: &'a [u8], version: u32) -> $crate::error::Result<Self::Model> {
                if version != 0 {
                    return Err($crate::error::DsotError::DataFormatError(format!("Entity {} => Invalid version: {}", stringify!($name), version)));
                }

                bincode1::deserialize(data)
                    .map_err(|e| $crate::error::DsotError::DeserializationError(e))
            }

            fn serialize(&self) -> $crate::error::Result<Vec<u8>> {
                let data: Vec<u8> = bincode1::serialize(self)
                    .map_err(|e| $crate::error::DsotError::SerializationError(e))?;

                Ok(data)
            }
        }
    };
}

macro_rules! declare_model_de {
    ($minor:ident, $major:ident, $version: literal) => {
        impl $crate::storage::BinModel for $major {
            type Model = $major;

            fn deserialize<'a>(data: &'a [u8]) -> $crate::error::Result<Self::Model> {
                let data = parse_data(data)?;
                Self::Model::deserialize_version(data.data, data.version)
            }

            fn deserialize_version<'a>(data: &'a [u8], version: u32) -> $crate::error::Result<Self::Model> {
                match version {
                    v if v < $version => {
                        let model = $minor::deserialize_version(data, version)?;
                        Ok(model.into())
                    },
                    v if v == $version => {
                        bincode1::deserialize(data)
                            .map_err(|e| $crate::error::DsotError::DeserializationError(e))
                    },
                    _ => Err($crate::error::DsotError::DataFormatError(format!("Entity {} => Invalid version: {}", stringify!($major), version))),
                }
            }

            fn serialize(&self) -> $crate::error::Result<Vec<u8>> {
                let data: Vec<u8> = bincode1::serialize(self)
                    .map_err(|e| $crate::error::DsotError::SerializationError(e))?;

                Ok(data)
            }
        }
    };
}

macro_rules! declare_all_model_de {
    ($version: literal: $name: ident) => {};
    ($version_prev: literal: $name_prev: ident, $version: literal: $name: ident $(, $v: literal: $n:ident)*) => {
        $crate::storage::macros::bin_model::declare_model_de!($name_prev, $name, $version);

        declare_all_model_de!($version: $name $(, $v: $n)*);
    };
}


macro_rules! declare_type {
    ($name: ident, $type: ident, $($rest: ident),*) => {
        declare_type!($name, $($rest),*);
    };
    ($name: ident, $type: ident) => {
        pub type $name = $type;
    };
}

#[macro_export]
macro_rules! declare_model {
    ($name: ident {
        $($version: literal: $type: ident),*
    }) => {
        $crate::storage::macros::bin_model::declare_type!($name, $($type),*);

        $crate::storage::macros::bin_model::declare_model_de_v0!($($type),*);

        $crate::storage::macros::bin_model::declare_all_model_de!($($version: $type),*);
    };
}

pub(crate) use declare_type;
pub(crate) use declare_all_model_de;
pub(crate) use declare_model_de;
pub(crate) use declare_model_de_v0;

#[cfg(test)]
mod tests {

}
