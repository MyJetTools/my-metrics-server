use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app_ctx::AppContext, db::HourStatisticsDto, events_amount_by_hour::StatisticsByHour};

use super::models::*;

#[my_http_server::macros::http_route(
    method: "GET",
    route: "/ui/GetServices",
    controller: "ui",
    description: "Get services overview",
    summary: "Get services overview",
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
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let now = DateTimeAsMicroseconds::now();

    let services = crate::flows::get_hour_app_statistics(&action.app, now.into()).await;

    let result = GetServicesResponse { services };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}

impl From<StatisticsByHour> for ServiceHttpModel {
    fn from(value: StatisticsByHour) -> Self {
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
