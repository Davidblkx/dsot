use iroh::{Endpoint, protocol::RouterBuilder};

pub async fn build_router(_builder: super::NetworkBuilder, endpoint: Endpoint) -> RouterBuilder {
    RouterBuilder::new(endpoint)
}
