use std::sync::Arc;

use axum::extract::{Json, State};
use dsot_core::model::entities::inbox::Inbox;
use dsot_runtime::{InboxInsertValue, InboxItems, Runtime};
use uuid::Uuid;

use crate::error::{HttpError, HttpResult};

pub fn register_inbox_routes(router: axum::Router<Arc<Runtime>>) -> axum::Router<Arc<Runtime>> {
    router
        .route("/api/v1/inbox/list", axum::routing::post(list_inbox))
        .route("/api/v1/inbox/update", axum::routing::post(update_inbox))
        .route("/api/v1/inbox/create", axum::routing::post(add_inbox))
        .route("/api/v1/inbox/delete", axum::routing::post(delete_inbox))
}

#[derive(serde::Deserialize)]
struct ListInboxParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_inbox(
    State(runtime): State<Arc<Runtime>>,
    Json(params): Json<ListInboxParams>,
) -> HttpResult<Vec<Inbox>> {
    let limit = params.limit.unwrap_or(100).max(0);
    let offset = params.offset.unwrap_or(0).max(0);
    let inbox = runtime.list_inbox(limit, offset).await;
    HttpError::handle_runtime(inbox)
}

async fn update_inbox(
    State(runtime): State<Arc<Runtime>>,
    Json(params): Json<Inbox>,
) -> HttpResult<bool> {
    let res = runtime.update_inbox(params).await;
    HttpError::handle_runtime(res)
}

async fn add_inbox(
    State(runtime): State<Arc<Runtime>>,
    Json(params): Json<InboxInsertValue>,
) -> HttpResult<Uuid> {
    let res = runtime.add_inbox(params).await;
    HttpError::handle_runtime(res)
}

async fn delete_inbox(
    State(runtime): State<Arc<Runtime>>,
    Json(params): Json<Inbox>,
) -> HttpResult<bool> {
    let res = runtime.delete_inbox(params.id).await;
    HttpError::handle_runtime(res)
}
