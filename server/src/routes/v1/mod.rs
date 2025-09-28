use axum::Router;

mod inbox;
mod users;

pub fn register_v1(
    router: Router<std::sync::Arc<dsot_runtime::Runtime>>,
) -> Router<std::sync::Arc<dsot_runtime::Runtime>> {
    let router = users::register_users_routes(router);
    let router = inbox::register_inbox_routes(router);
    router
}
