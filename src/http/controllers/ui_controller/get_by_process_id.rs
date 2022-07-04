use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app_ctx::AppContext;

use super::models::*;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/ui/GetByProcessId",
    controller: "ui",
    description: "Get by process id",
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
    let events = {
        let read_access = action.app.metrics.lock().await;
        read_access.get_by_process_id(http_input.process_id)
    };

    let mut metrics = Vec::new();

    for event in events {
        metrics.push(MetricByProcessModel {
            data: event.event_data.to_string(),
            started: event.started.unix_microseconds,
            duration: event.get_duration_mcs(),
            success: event.success.clone(),
            error: event.fail.clone(),
            ip: event.ip.clone(),
        });
    }

    let result = MetricsByProcessResponse { metrics };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
