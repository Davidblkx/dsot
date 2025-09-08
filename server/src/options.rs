use std::sync::Arc;

use dsot_runtime::Runtime;

pub struct ServerOptions {
    pub runtime: Arc<Runtime>,
    pub port: u16,
}
