mod devices;
mod user;

pub use devices::*;
pub use user::*;

pub trait Repository: UserRepository + DeviceRepository + Send + Sync {}
