use crate::{
    caches::AggregatedMetricsByServiceCache,
    postgres::{MetricsPostgresRepo, StatisticsRepo},
    settings::SettingsReader,
};
use rust_extensions::AppStates;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::ToWriteQueue;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
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
    pub async fn new(settings_reader: Arc<SettingsReader>) -> AppContext {
        AppContext {
            to_write_queue: ToWriteQueue::new(),
            app_states: Arc::new(AppStates::create_initialized()),
            process_id: uuid::Uuid::new_v4().to_string(),
            repo: MetricsPostgresRepo::new(settings_reader.clone()).await,
            statistics_repo: StatisticsRepo::new(settings_reader.clone()).await,
            settings_reader,
            metrics_cache: Mutex::new(AggregatedMetricsByServiceCache::new()),
        }
    }
}
