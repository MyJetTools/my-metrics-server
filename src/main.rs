use std::{sync::Arc, time::Duration};

use background::GcMetricsTimer;
use rust_extensions::MyTimer;

mod app_ctx;
mod background;
mod http;
mod services_metrics;

#[tokio::main]
async fn main() {
    let app = app_ctx::AppContext::new();

    let app = Arc::new(app);

    let mut http_server = http::start_up::setup_server(&app, 8000);

    let mut gc_timer = MyTimer::new(Duration::from_secs(10));

    gc_timer.register_timer("GcTimer", Arc::new(GcMetricsTimer::new(app.clone())));

    gc_timer.start(app.app_states.clone(), app.logger.clone());

    http_server.start(app.app_states.clone(), app.logger.clone());

    app.app_states.wait_until_shutdown().await;
}
