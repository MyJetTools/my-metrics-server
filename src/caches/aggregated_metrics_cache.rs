use std::collections::{BTreeMap, HashMap};

use tokio::sync::Mutex;

use crate::postgres::dto::MetricDto;

use super::MetricsByActionName;

#[derive(Debug)]
pub struct ServiceInfo {
    pub avg: i64,
    pub amount: i64,
}

#[derive(Debug)]
pub struct ActionInfo {
    pub max: i64,
    pub min: i64,
    pub avg: i64,
    pub success: i64,
    pub errors: i64,
}

pub struct AggregatedMetricsByServiceCache {
    data: Mutex<HashMap<String, MetricsByActionName>>,
}

impl AggregatedMetricsByServiceCache {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub async fn update(&self, events: &[MetricDto]) {
        let mut write_access = self.data.lock().await;

        for event in events {
            if !write_access.contains_key(&event.name) {
                write_access.insert(event.name.to_string(), MetricsByActionName::new());
            }

            write_access.get_mut(&event.name).unwrap().update(event);
        }
    }

    pub async fn get_services(&self) -> BTreeMap<String, ServiceInfo> {
        let mut result = BTreeMap::new();

        let read_access = self.data.lock().await;

        for (service_name, data) in read_access.iter() {
            result.insert(service_name.to_string(), data.get_avg());
        }

        result
    }

    pub async fn get_actions_statistics(
        &self,
        service_name: &str,
    ) -> Option<BTreeMap<String, ActionInfo>> {
        let read_access = self.data.lock().await;

        if let Some(data) = read_access.get(service_name) {
            return Some(data.get_action_info());
        }

        None
    }
}
