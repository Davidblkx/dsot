mod v1;

use axum::Router;

pub fn create_app_routes() -> Router<std::sync::Arc<dsot_runtime::Runtime>> {
    v1::register_v1(Router::new())
}
