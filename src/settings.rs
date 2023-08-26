use std::time::Duration;

use my_postgres::PostgresSettings;
use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "PostgresConnString")]
    pub postgres_conn_string: String,
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: String,

    #[serde(rename = "MinEventsToKeepBeforeGC")]
    pub min_events_to_keep_before_gc: usize,
}

impl SettingsReader {
    pub async fn get_min_events_to_keep_before_gc(&self) -> usize {
        let read_access = self.settings.read().await;
        read_access.min_events_to_keep_before_gc
    }

    pub async fn get_duration_before_now_to_gc(&self) -> Duration {
        let read_access = self.settings.read().await;
        rust_extensions::duration_utils::parse_duration(read_access.retention_period.as_str())
            .unwrap()
    }
}

#[async_trait::async_trait]
impl PostgresSettings for SettingsReader {
    async fn get_connection_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.postgres_conn_string.clone()
    }
}
