use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app_ctx::AppContext;

use super::models::*;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/ui/GetByProcessId",
    controller: "ui",
    description: "Get by process id",
    summary: "Get by process id",
    input_data: "GetByProcessIdRequest",
    result:[
        {status_code: 200, description: "List of apps", model="MetricsByProcessResponse"},
    ]
)]
pub struct GetByProcessIdAction {
    app: Arc<AppContext>,
}

impl GetByProcessIdAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetByProcessIdAction,
    http_input: GetByProcessIdRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let events = action
        .app
        .repo
        .get_by_process_id(http_input.hour_key.into(), http_input.process_id)
        .await;

    let mut metrics = Vec::with_capacity(events.len());

    for event in events {
        metrics.push(event.into());
    }

    let result = MetricsByProcessResponse { metrics };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
