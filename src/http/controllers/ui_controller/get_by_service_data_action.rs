use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey};

use crate::app_ctx::AppContext;

use super::models::*;

#[my_http_server::macros::http_route(
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
    let hour_key: IntervalKey<HourKey> = input_data.hour_key.into();

    let from_started = if input_data.from_second_within_hour == 0 {
        None
    } else {
        let mut dt: DateTimeAsMicroseconds = hour_key.try_to_date_time().unwrap();
        dt.add_seconds(input_data.from_second_within_hour);
        Some(dt.unix_microseconds)
    };
    let events = action
        .app
        .repo
        .get_by_service_name(
            input_data.hour_key.into(),
            &input_data.id,
            &input_data.data,
            input_data.client_id.as_deref(),
            from_started,
        )
        .await;

    let mut metrics = Vec::new();

    for event in events {
        metrics.push(event.into());
    }

    let result = MetricsResponse { metrics };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
