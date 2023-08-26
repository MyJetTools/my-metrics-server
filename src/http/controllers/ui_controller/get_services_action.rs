use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;

use super::models::*;

#[my_http_server_swagger::http_route(
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
    let mut from = DateTimeAsMicroseconds::now();

    from.add_days(-1);

    let overview = action.app.repo.get_services(from).await;

    let mut services = Vec::with_capacity(overview.len());

    for dto in overview {
        services.push(ServiceModel {
            id: dto.name,
            avg: dto.avg.get_value() as i32,
        });
    }

    let result = GetServicesResponse { services };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
