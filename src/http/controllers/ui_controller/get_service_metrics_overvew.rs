use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app_ctx::AppContext;

use super::models::*;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/ui/GetServiceOverview",
    controller: "ui",
    description: "Get Service Metrics Overview",
    summary: "Get Service Metrics Overview",
    input_data: "GetServiceMetricsOverview",
    result:[
        {status_code: 200, description: "List of apps", model="GetServiceOverviewResponse"},
    ]
)]
pub struct GetServiceMetricsOvervewAction {
    app: Arc<AppContext>,
}

impl GetServiceMetricsOvervewAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetServiceMetricsOvervewAction,
    input_data: GetServiceMetricsOverview,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let services_overview = {
        let read_access = action.app.metrics.lock().await;
        read_access.get_service_overview(&input_data.id)
    };

    let mut data = Vec::new();

    for (_, domain_model) in services_overview {
        data.push(ServiceOverviewContract {
            data: domain_model.data,
            min: domain_model.min,
            max: domain_model.max,
            avg: domain_model.avg,
            success: domain_model.success,
            error: domain_model.error,
            total: domain_model.total,
        });
    }

    let result = GetServiceOverviewResponse { data };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
