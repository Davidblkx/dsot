use std::sync::Arc;

use axum::extract::{Query, State};
use dsot_core::model::entities::inbox::Inbox;
use dsot_runtime::{InboxItems, Runtime};

use crate::error::{HttpError, HttpResult};

pub fn register_inbox_routes(router: axum::Router<Arc<Runtime>>) -> axum::Router<Arc<Runtime>> {
    router.route("/api/v1/inbox/list", axum::routing::post(list_inbox))
}

#[derive(serde::Deserialize)]
struct ListInboxParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_inbox(
    Query(params): Query<ListInboxParams>,
    State(runtime): State<Arc<Runtime>>,
) -> HttpResult<Vec<Inbox>> {
    let limit = params.limit.unwrap_or(100).max(0);
    let offset = params.offset.unwrap_or(0).max(0);
    let inbox = runtime.list_inbox(limit, offset).await;
    HttpError::handle_runtime(inbox)
}
