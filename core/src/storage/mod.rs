mod transaction;
mod handler;
mod ser;
mod entity;
mod storage;

pub mod macros;

pub use transaction::StorageTransaction;
pub use handler::StorageHandler;
pub use ser::{BinModelData, BinModel, parse_data_with_version, serialize_data_with_version};
pub use entity::{StorageEntity, HasBytes};
pub use storage::Storage;

pub use bincode1::deserialize as deserialize_bincode1;
pub use bincode1::serialize as serialize_bincode1;
