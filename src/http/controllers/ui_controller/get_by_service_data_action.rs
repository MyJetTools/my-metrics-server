use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app_ctx::AppContext;

use super::models::*;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/ui/GetByServiceData",
    controller: "ui",
    description: "Get Service Metrics Overview",
    summary: "Get Service Metrics Overview",
    input_data: "GetByServiceDataRequest",
    result:[
        {status_code: 200, description: "List of apps", model="GetServiceOverviewResponse"},
    ]
)]
pub struct GetByServiceDataAction {
    app: Arc<AppContext>,
}

impl GetByServiceDataAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetByServiceDataAction,
    input_data: GetByServiceDataRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let events = action
        .app
        .repo
        .get_by_service_name(&input_data.id, &input_data.data)
        .await;

    let mut metrics = Vec::new();

    for event in events {
        metrics.push(event.into());
    }

    let result = MetricsResponse { metrics };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
