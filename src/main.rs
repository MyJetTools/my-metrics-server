use std::{sync::Arc, time::Duration};

use background::{GcMetricsTimer, MetricsWriter};
use rust_extensions::MyTimer;

mod app_ctx;
mod background;
mod http;
mod postgres;
mod settings;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".my-telemetry").await;

    let settings_reader = Arc::new(settings_reader);
    let app = app_ctx::AppContext::new(settings_reader).await;

    let app = Arc::new(app);

    let mut http_server = http::start_up::setup_server(&app, 8000);

    let mut gc_timer = MyTimer::new(Duration::from_secs(10));

    gc_timer.register_timer("GcTimer", Arc::new(GcMetricsTimer::new(app.clone())));

    gc_timer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    let metrics_writer = MetricsWriter::new(app.clone());
    app.to_write_queue
        .events_loop
        .register_event_loop(Arc::new(metrics_writer))
        .await;

    app.to_write_queue
        .events_loop
        .start(app.app_states.clone(), my_logger::LOGGER.clone())
        .await;

    app.app_states.wait_until_shutdown().await;

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
