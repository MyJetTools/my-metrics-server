use crate::{
    caches::AggregatedMetricsByServiceCache, postgres::MetricsPostgresRepo,
    settings::SettingsReader,
};
use rust_extensions::AppStates;
use std::sync::Arc;

use super::ToWriteQueue;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub process_id: String,
    pub repo: MetricsPostgresRepo,
    pub settings_reader: Arc<SettingsReader>,
    pub to_write_queue: ToWriteQueue,
    pub metrics_cache: AggregatedMetricsByServiceCache,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> AppContext {
        AppContext {
            to_write_queue: ToWriteQueue::new(),
            app_states: Arc::new(AppStates::create_initialized()),
            process_id: uuid::Uuid::new_v4().to_string(),
            repo: MetricsPostgresRepo::new(settings_reader.clone()).await,
            settings_reader,
            metrics_cache: AggregatedMetricsByServiceCache::new(),
        }
    }
}
