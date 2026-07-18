use iroh::protocol::RouterBuilder;

use super::builder::NetworkBuilder;
use crate::error::Result;

mod traits;

pub mod info;

pub use traits::DsotProtocolHandler;

pub async fn add_routes(
    mut router: RouterBuilder,
    builder: NetworkBuilder,
) -> Result<RouterBuilder> {
    router = info::InfoProtocol::new(&builder).register_router(router);

    Ok(router)
}
