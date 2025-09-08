pub mod error;
mod options;

use axum::{Router, routing::get};
use std::sync::Arc;

use axum::extract::State;
pub use options::ServerOptions;

use crate::error::ServerResult;

pub async fn run_server<'a>(options: ServerOptions) -> ServerResult<()> {
    let addr = format!("0.0.0.0:{}", options.port);

    let app = Router::new()
        .route("/", get(handle_connection))
        .with_state(options.runtime);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    log::info!("Server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_connection<'a>(State(state): State<Arc<dsot_runtime::Runtime>>) -> &'static str {
    state.version
}
