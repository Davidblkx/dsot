use iroh::protocol::Router;

use super::builder::NetworkBuilder;

#[derive(Debug, Clone, Default)]
pub enum NetworkState {
    #[default]
    Closed,
    Ready(NetworkBuilder),
    Open(Router),
}
