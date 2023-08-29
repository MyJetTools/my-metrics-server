use std::{collections::BTreeMap, sync::Arc};

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

    let mut sorted = BTreeMap::new();

    for dto in overview {
        sorted.insert(
            dto.name.clone(),
            ServiceModel {
                id: dto.name,
                avg: dto.avg.get_value() as i32,
            },
        );
    }

    let mut services = Vec::with_capacity(sorted.len());

    for (_, service) in sorted {
        services.push(service);
    }

    let result = GetServicesResponse { services };

    return HttpOutput::as_json(result).into_ok_result(true).into();
}
