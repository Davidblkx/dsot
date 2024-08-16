use std::collections::HashSet;

use native_model::native_model;
use serde::{Deserialize, Serialize};

macro_rules! impl_hashset {
    ($name:ident, $type:expr) => {
        paste::paste! {
            impl $name {
                pub fn new(values: HashSet<$type>) -> Self {
                    Self { values }
                }

                pub fn empty() -> Self {
                    Self {
                        values: HashSet::new(),
                    }
                }

                pub fn to_binary(&self) -> crate::error::Result<Vec<u8>> {
                    let v = native_model::encode(self)?;
                    Ok(v)
                }

                pub fn from_binary(data: Vec<u8>) -> crate::error::Result<$name> {
                    let v = native_model::decode::<$name>(data)?;
                    Ok(v.0)
                }
            }
        }
    };
}

macro_rules! impl_vec {
    ($name:ident, $type:expr) => {
        paste::paste! {
            impl $name {
                pub fn new(values: Vec<$type>) -> Self {
                    Self { values }
                }

                pub fn empty() -> Self {
                    Self {
                        values: Vec::new(),
                    }
                }

                pub fn to_binary(&self) -> crate::error::Result<Vec<u8>> {
                    let v = native_model::encode(self)?;
                    Ok(v)
                }

                pub fn from_binary(data: Vec<u8>) -> crate::error::Result<$name> {
                    let v = native_model::decode::<$name>(data)?;
                    Ok(v.0)
                }
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[native_model(id = 101, version = 1)]
pub struct HashSetString { pub values: HashSet<String> }
impl_hashset!(HashSetString, String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[native_model(id = 201, version = 1)]
pub struct VecU64 { pub values: Vec<u64> }
impl_vec!(VecU64, u64);
