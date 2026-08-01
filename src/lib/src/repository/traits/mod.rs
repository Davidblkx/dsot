mod devices;
mod inbox;
mod user;

pub use devices::*;
pub use inbox::*;
pub use user::*;

pub trait Repository: UserRepository + DeviceRepository + InboxRepository + Send + Sync {}
