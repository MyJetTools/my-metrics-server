use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::ignore_events::IgnoreEvents;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IgnoreEvent {
    pub name: String,
    pub data: String,
}

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "DbPath")]
    pub db_path: String,
    #[serde(rename = "RetentionPeriod")]
    pub retention_period: String,

    #[serde(rename = "MinEventsToKeepBeforeGC")]
    pub min_events_to_keep_before_gc: usize,

    #[serde(rename = "IgnoreEvents")]
    pub ignore_events: Vec<IgnoreEvent>,
}

impl SettingsReader {
    pub async fn get_ignore_events(&self) -> IgnoreEvents {
        let read_access = self.settings.read().await;

        IgnoreEvents::new(read_access.ignore_events.clone())
    }

    pub async fn get_db_path(&self) -> String {
        let read_access = self.settings.read().await;

        let db_path = if read_access.db_path.ends_with(std::path::MAIN_SEPARATOR) {
            &read_access.db_path[..read_access.db_path.len() - 1]
        } else {
            read_access.db_path.as_str()
        };

        rust_extensions::file_utils::format_path(db_path).to_string()
    }

    pub async fn get_db_file_prefix(&self, file_name: &str) -> String {
        let read_access = self.settings.read().await;

        let mut result = if read_access.db_path.starts_with("~") {
            read_access
                .db_path
                .replace("~", &std::env::var("HOME").unwrap())
        } else {
            read_access.db_path.clone()
        };

        if !result.ends_with(std::path::MAIN_SEPARATOR) {
            result.push(std::path::MAIN_SEPARATOR)
        }

        result.push_str(file_name);

        result
    }
    /*
    pub async fn get_min_events_to_keep_before_gc(&self) -> usize {
        let read_access = self.settings.read().await;
        read_access.min_events_to_keep_before_gc
    }
    */
    pub async fn get_duration_before_now_to_gc(&self) -> Duration {
        let read_access = self.settings.read().await;
        rust_extensions::duration_utils::parse_duration(read_access.retention_period.as_str())
            .unwrap()
    }
}
