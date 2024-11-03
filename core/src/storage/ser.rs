use crate::error::{DsotError, Result};

/// Used to represent a binary model data with a version
#[derive(Debug)]
pub struct BinModelData<'a> {
    pub version: u32,
    pub data: &'a [u8],
}

/// Parse version from binary data
///
/// # Arguments
///
/// * `data` - A slice of binary data
pub fn parse_data_with_version<'a>(data: &'a [u8]) -> Result<BinModelData<'a>> {
    if data.len() < 5 {
        return Err(DsotError::DataFormatError("Data is too short".to_string()));
    }

    let version: [u8; 4] = data[..4].try_into().unwrap();

    let version = u32::from_be_bytes(version);
    let data = &data[4..];

    Ok(BinModelData { version, data })
}

/// Serialize data with version
///
/// # Arguments
///
/// * `version` - A version number
/// * `data` - A slice of binary data
pub fn serialize_data_with_version(version: u32, data: &[u8]) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(data.len() + 4);
    buf.extend_from_slice(&version.to_be_bytes());
    buf.extend_from_slice(data);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_with_version() {
        let data = vec![0, 0, 0, 1, 1, 2, 3, 4, 5];
        let result = parse_data_with_version(&data);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.version, 1);
        assert_eq!(result.data, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_data_with_version_short_data() {
        let data = vec![0, 0, 0, 1];
        let result = parse_data_with_version(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_data_with_version() {
        let data = vec![1, 2, 3, 4, 5];
        let result = serialize_data_with_version(1, &data);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec![0, 0, 0, 1, 1, 2, 3, 4, 5]);
    }
}
