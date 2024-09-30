use crate::error::Result;

/// A trait that defines the schema of a table.
///
/// It is used to define the table name, version, how to serialize/deserialize the value
/// and how to update the table version.
pub trait TableSchema {
    /// The type of the value that the table holds.
    type Value;
    /// The name of the table.
    fn table_name() -> &'static str;
    /// The version used to serialize the table values.
    fn version() -> u64;
    /// Get the serialized key of the value.
    fn get_key<'a>(value: &'a Self::Value) -> &'a [u8];
    /// Deserialize a byte array to a value.
    fn deserialize<'a>(version: u64, value: &'a [u8]) -> Result<Self::Value>;
    /// Serialize the value to a byte array.
    fn serialize<'a>(value: &'a Self::Value) -> Result<Vec<u8>>;
    /// Update the serialized version of the value.
    /// This is used to update the version of the value when the table version changes.
    /// The returned value should be the serialized value of the new version.
    /// If the value is not updated, return `None`.
    fn update_version<'a>(version: u64, value: &'a [u8]) -> Result<Option<Vec<u8>>>;
}
