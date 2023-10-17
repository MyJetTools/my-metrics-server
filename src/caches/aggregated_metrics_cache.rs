use std::collections::{BTreeMap, HashMap};

use tokio::sync::Mutex;

use crate::postgres::dto::MetricDto;

use super::MetricsByActionName;

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

    pub async fn get_services(&self) -> BTreeMap<String, i64> {
        let mut result = BTreeMap::new();

        let read_access = self.data.lock().await;

        for (service_name, data) in read_access.iter() {
            result.insert(service_name.to_string(), data.get_avg());
        }

        result
    }
}
