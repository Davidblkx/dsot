use crate::error::{DsotError, Result};

pub struct ModelV0 {
    pub id: uuid::Uuid,
}

pub struct ModelV1 {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<ModelV0> for ModelV1 {
    fn from(model: ModelV0) -> Self {
        ModelV1 {
            id: model.id,
            name: String::new(),
        }
    }
}

pub struct ModelV2 {
    pub id: uuid::Uuid,
    pub name: String,
    pub age: u32,
}

impl From<ModelV1> for ModelV2 {
    fn from(model: ModelV1) -> Self {
        ModelV2 {
            id: model.id,
            name: model.name,
            age: 0,
        }
    }
}

macro_rules! declare_type {
    ($name: ident, $type: ident, $($rest: ident),*) => {
        declare_type!($name, $($rest),*);
    };
    ($name: ident, $type: ident) => {
        pub type $name = $type;
    };
}

declare_type!(Model, ModelV0, ModelV1, ModelV2);

pub struct BinModelData<'a> {
    pub version: u32,
    pub data: &'a [u8],
}

pub fn parse_data<'a>(data: &'a [u8]) -> Result<BinModelData<'a>> {
    if data.len() < 5 {
        return Err(DsotError::DataFormatError("Data is too short".to_string()));
    }

    let version: [u8; 4] = data[..4].try_into()
        .map_err(|_| DsotError::DataFormatError("Invalid version data".to_string()))?;

    let version = u32::from_le_bytes(version);
    let data = &data[5..];

    Ok(BinModelData { version, data })
}

pub trait BinModel {
    type Model: Sized;

    fn deserialize<'a>(data: &'a [u8]) -> Result<Self::Model>;

    fn deserialize_version<'a>(data: &'a [u8], version: u32) -> Result<Self::Model>;
}

macro_rules! declare_model_de {
    ($minor:ident, $major:ident) => {

    };
}
