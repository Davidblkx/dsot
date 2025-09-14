pub mod error;
mod options;
mod routes;

use routes::create_app_routes;

pub use options::ServerOptions;

use crate::error::ServerResult;

pub async fn run_server(options: ServerOptions) -> ServerResult<()> {
    let addr = format!("0.0.0.0:{}", options.port);

    let app = create_app_routes().with_state(options.runtime);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    log::info!("Server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
