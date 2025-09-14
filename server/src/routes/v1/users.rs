use std::sync::Arc;

use axum::extract::State;
use dsot_core::model::entities::user::User;
use dsot_runtime::{Runtime, Users};

use crate::error::{HttpError, HttpResult};

pub fn register_users_routes(router: axum::Router<Arc<Runtime>>) -> axum::Router<Arc<Runtime>> {
    router.route("/api/v1/users/list", axum::routing::post(list_users))
}

async fn list_users(State(runtime): State<Arc<Runtime>>) -> HttpResult<Vec<User>> {
    let users = runtime.list_users().await;
    HttpError::handle_runtime(users)
}
