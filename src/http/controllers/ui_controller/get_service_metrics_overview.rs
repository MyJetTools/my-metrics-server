use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;

use super::models::*;

#[my_http_server::macros::http_route(
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
pub struct GetServiceMetricsOverviewAction {
    app: Arc<AppContext>,
}

impl GetServiceMetricsOverviewAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetServiceMetricsOverviewAction,
    input_data: GetServiceMetricsOverview,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let mut from = DateTimeAsMicroseconds::now();

    from.add_days(-1);

    let dto_data = action
        .app
        .statistics_repo
        .get_aggregated_statistics_of_service(&input_data.id)
        .await;

    let mut data = Vec::new();

    for dto in dto_data {
        data.push(ServiceOverviewContract {
            data: dto.data,
            min: dto.min,
            max: dto.max,
            avg: dto.avg,
            success: dto.success_amount,
            error: dto.errors_amount,
            total: dto.amount,
        });
    }

    let result = GetServiceOverviewResponse { data };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
