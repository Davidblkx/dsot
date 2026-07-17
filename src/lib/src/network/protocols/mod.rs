use iroh::protocol::RouterBuilder;

use super::builder::NetworkBuilder;
use crate::error::Result;

mod traits;

pub mod info;

pub use traits::DsotProtocolHandler;

pub async fn add_routes(
    router: RouterBuilder,
    _net_builder: NetworkBuilder,
) -> Result<RouterBuilder> {
    Ok(router)
}
