use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app_ctx::AppContext;

use super::models::NewMetricsEvent;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/metric_event",
    controller: "api",
    description: "New Metric Event",
    input_data: "NewMetricsEvent",

)]
pub struct PostMetricAction {
    app: Arc<AppContext>,
}

impl PostMetricAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &PostMetricAction,
    input_data: NewMetricsEvent,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let mut metrics_write_access = action.app.metrics.lock().await;

    for metrics in input_data.metrics {
        metrics_write_access.new_event(
            metrics.service_name,
            metrics.event_data,
            metrics.started,
            metrics.ended,
            metrics.process_id,
            metrics.success,
            metrics.fail,
        )
    }

    return HttpOutput::Empty.into_ok_result(true).into();
}
