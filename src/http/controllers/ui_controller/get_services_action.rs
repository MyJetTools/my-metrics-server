use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::{app_ctx::AppContext, caches::AppDurationStatistics, db::HourStatisticsDto};

use super::models::*;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/ui/GetServices",
    controller: "ui",
    description: "Get services overview",
    summary: "Get services overview",
    input_data:GetServicesHttpInput,
    result:[
        {status_code: 200, description: "List of apps", model="GetServicesResponse"},
    ]
)]
pub struct GetServicesAction {
    app: Arc<AppContext>,
}

impl GetServicesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetServicesAction,
    input_data: GetServicesHttpInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let services =
        crate::flows::get_hour_app_statistics(&action.app, input_data.hour_key.into()).await;

    let result = GetServicesResponse { services };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}

impl From<AppDurationStatistics> for ServiceHttpModel {
    fn from(value: AppDurationStatistics) -> Self {
        Self {
            id: value.name,
            avg: value.duration_micros / value.amount,
            amount: value.amount,
        }
    }
}

impl From<HourStatisticsDto> for ServiceHttpModel {
    fn from(value: HourStatisticsDto) -> Self {
        Self {
            id: value.app,
            avg: value.duration_micros / value.amount,
            amount: value.amount,
        }
    }
}
