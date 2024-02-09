use std::time::Duration;

use my_postgres::PostgresSettings;
use serde::{Deserialize, Serialize};

use crate::ignore_events::IgnoreEvents;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IgnoreEvent {
    pub name: String,
    pub data: String,
}

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "PostgresConnString")]
    pub postgres_conn_string: String,
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

#[async_trait::async_trait]
impl PostgresSettings for SettingsReader {
    async fn get_connection_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.postgres_conn_string.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let model = SettingsModel {
            postgres_conn_string: "postgres://postgres:postgres@localhost:5432/postgres"
                .to_string(),
            retention_period: "1h".to_string(),
            min_events_to_keep_before_gc: 10,
            ignore_events: vec![IgnoreEvent {
                name: "test".to_string(),
                data: "[SignalR] ping".to_string(),
            }],
        };

        let result = serde_yaml::to_string(&model).unwrap();

        println!("{}", result);
    }
}
