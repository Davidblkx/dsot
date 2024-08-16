use crate::error::{Result, DsotError};

pub fn new_uuid() -> Vec<u8> {
    uuid7::new_v7().as_bytes().to_vec()
}

pub fn parse_uuid(v: &Vec<u8>) -> Result<uuid::Uuid> {
    let x: [u8; 16] = v.as_slice().try_into().map_err(|_| DsotError::InvalidUuidLength)?;
    Ok(uuid::Uuid::from_bytes(x))
}

pub fn uuid_to_vec(u: &uuid::Uuid) -> Vec<u8> {
    u.as_bytes().to_vec()
}
