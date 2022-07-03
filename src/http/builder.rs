use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::app_ctx::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new();

    result.register_post_action(Arc::new(
        super::controllers::metric_event::PostMetricAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        super::controllers::ui_controller::GetServicesAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        super::controllers::ui_controller::GetServiceMetricsOvervewAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        super::controllers::ui_controller::GetByServiceDataAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        super::controllers::home_controller::IndexAction::new(app.clone()),
    ));

    result
}
