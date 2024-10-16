use crate::{
    caches::StatisticsByAppAndData,
    db::{HourAppDataStatisticsRepo, HourStatisticsRepo, MetricsRepo},
    events_amount_by_hour::EventAmountsByHour,
    process_id_user_id_links::ProcessIdUserIdLinks,
    settings::SettingsReader,
    to_write_queue::ToWriteQueue,
};
use rust_extensions::AppStates;
use std::sync::Arc;
use tokio::sync::Mutex;

//pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const METRICS_FILE_PREFIX: &'static str = "metrics";

pub struct StatisticsCache {
    pub event_amount_by_hours: EventAmountsByHour,
    pub statistics_by_app_and_data: StatisticsByAppAndData,
    pub process_id_user_id_links: ProcessIdUserIdLinks,
}

impl StatisticsCache {
    pub fn new() -> Self {
        Self {
            statistics_by_app_and_data: StatisticsByAppAndData::new(),
            event_amount_by_hours: EventAmountsByHour::new(),
            process_id_user_id_links: ProcessIdUserIdLinks::new(),
        }
    }
}

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub process_id: String,
    pub repo: MetricsRepo,

    pub settings_reader: Arc<SettingsReader>,
    pub to_write_queue: ToWriteQueue,
    pub cache: Mutex<StatisticsCache>,
    pub hour_statistics_repo: HourStatisticsRepo,
    pub hour_app_data_statistics_repo: HourAppDataStatisticsRepo,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> AppContext {
        let repo_file_name = settings_reader
            .get_db_file_prefix(METRICS_FILE_PREFIX)
            .await;
        let statistic_db_file_name = settings_reader
            .get_db_file_prefix("h_app_statistics.db")
            .await;

        let h_statistic_db_file_name = settings_reader.get_db_file_prefix("h_statistics.db").await;

        AppContext {
            to_write_queue: ToWriteQueue::new(),
            app_states: Arc::new(AppStates::create_initialized()),
            process_id: uuid::Uuid::new_v4().to_string(),
            repo: MetricsRepo::new(repo_file_name).await,
            hour_app_data_statistics_repo: HourAppDataStatisticsRepo::new(statistic_db_file_name)
                .await,
            settings_reader,
            hour_statistics_repo: HourStatisticsRepo::new(h_statistic_db_file_name).await,
            cache: Mutex::new(StatisticsCache::new()),
        }
    }
}
