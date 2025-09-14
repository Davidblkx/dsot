use axum::Router;

mod users;

pub fn register_v1(
    router: Router<std::sync::Arc<dsot_runtime::Runtime>>,
) -> Router<std::sync::Arc<dsot_runtime::Runtime>> {
    let router = users::register_users_routes(router);
    router
}
