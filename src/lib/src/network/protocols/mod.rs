use iroh::protocol::RouterBuilder;

use super::builder::NetworkBuilder;
use crate::error::Result;

mod traits;

pub mod info;
pub mod server;

pub use traits::DsotProtocolHandler;

pub async fn add_routes(
    mut router: RouterBuilder,
    builder: NetworkBuilder,
) -> Result<RouterBuilder> {
    router = info::InfoProtocol::new(&builder).register_router(router);

    router = server::add_server_routes(router, &builder).await?;

    Ok(router)
}
