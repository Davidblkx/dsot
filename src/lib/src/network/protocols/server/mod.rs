use iroh::protocol::RouterBuilder;

use crate::{
    error::Result,
    network::{DsotProtocolHandler, builder::NetworkBuilder},
};

mod token_validation;
pub mod users;

pub use token_validation::{NetworkValidator, TokenValidator};

pub async fn add_server_routes(
    mut router: RouterBuilder,
    builder: &NetworkBuilder,
) -> Result<RouterBuilder> {
    if !builder.config.value.use_server {
        return Ok(router);
    }

    router = users::UsersProtocol::new(builder).register_router(router);

    Ok(router)
}
