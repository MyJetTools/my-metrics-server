use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::{app_ctx::AppContext, caches::AppDataHourStatistics, db::HourAppDataStatisticsDto};

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
    let data = crate::flows::get_hour_app_data_statistics(
        &action.app,
        input_data.hour_key.into(),
        &input_data.id,
    )
    .await;

    let result = GetServiceOverviewResponse { data };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}

impl From<AppDataHourStatistics> for ServiceOverviewContract {
    fn from(value: AppDataHourStatistics) -> Self {
        let total = value.success_amount + value.errors_amount;
        ServiceOverviewContract {
            data: value.data,
            min: value.min,
            max: value.max,
            avg: value.sum_of_duration / total,
            success: value.success_amount,
            error: value.errors_amount,
            total,
        }
    }
}

impl From<HourAppDataStatisticsDto> for ServiceOverviewContract {
    fn from(value: HourAppDataStatisticsDto) -> Self {
        let total = value.success_amount + value.errors_amount;
        ServiceOverviewContract {
            data: value.data,
            min: value.min,
            max: value.max,
            avg: value.sum_of_duration / total,
            success: value.success_amount,
            error: value.errors_amount,
            total,
        }
    }
}
