use std::{sync::Arc, time::Duration};

use background::{GcMetricsTimer, MetricsWriter, SaveStatisticsTimer};
use rust_extensions::{events_loop::EventsLoop, MyTimer};

mod app_ctx;
mod background;
mod caches;
mod flows;
mod grpc_server;
mod http;
mod postgres;
mod settings;

pub mod writer_grpc {
    tonic::include_proto!("writer");
}

pub mod reader_grpc {
    tonic::include_proto!("reader");
}

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".my-telemetry").await;

    let settings_reader = Arc::new(settings_reader);

    let mut events_loop = EventsLoop::new("MetricsWriter".to_string(), my_logger::LOGGER.clone());

    let app = app_ctx::AppContext::new(settings_reader, events_loop.get_publisher()).await;

    let app = Arc::new(app);

    let mut http_server = http::start_up::setup_server(&app, 8000);

    let mut gc_timer =
        MyTimer::new_with_execute_timeout(Duration::from_secs(10), Duration::from_secs(60 * 5));

    gc_timer.register_timer("GcTimer", Arc::new(GcMetricsTimer::new(app.clone())));

    gc_timer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    let mut save_statistics_timer = MyTimer::new(Duration::from_secs(1));

    save_statistics_timer.register_timer(
        "SaveStatisticsTimer",
        Arc::new(SaveStatisticsTimer::new(app.clone())),
    );

    save_statistics_timer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    let metrics_writer = MetricsWriter::new(app.clone());
    events_loop.register_event_loop(Arc::new(metrics_writer));

    events_loop.start(app.app_states.clone());

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());

    grpc_server::start(&app, 8888);
    app.app_states.wait_until_shutdown().await;
}
