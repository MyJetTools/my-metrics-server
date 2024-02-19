use crate::{
    caches::AggregatedMetricsByServiceCache,
    postgres::{MetricsPostgresRepo, StatisticsRepo},
    settings::SettingsReader,
};
use rust_extensions::{events_loop::EventsLoopPublisher, AppStates};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::ToWriteQueue;

//pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub process_id: String,
    pub repo: MetricsPostgresRepo,
    pub statistics_repo: StatisticsRepo,
    pub settings_reader: Arc<SettingsReader>,
    pub to_write_queue: ToWriteQueue,
    pub metrics_cache: Mutex<AggregatedMetricsByServiceCache>,
}

impl AppContext {
    pub async fn new(
        settings_reader: Arc<SettingsReader>,
        events_loop_publisher: EventsLoopPublisher<()>,
    ) -> AppContext {
        let repo_file_name = settings_reader.get_db_file_name("metrics.db").await;
        let statistic_db_file_name = settings_reader.get_db_file_name("statistics.db").await;

        AppContext {
            to_write_queue: ToWriteQueue::new(events_loop_publisher),
            app_states: Arc::new(AppStates::create_initialized()),
            process_id: uuid::Uuid::new_v4().to_string(),
            repo: MetricsPostgresRepo::new(repo_file_name).await,
            statistics_repo: StatisticsRepo::new(statistic_db_file_name).await,
            settings_reader,
            metrics_cache: Mutex::new(AggregatedMetricsByServiceCache::new()),
        }
    }
}
