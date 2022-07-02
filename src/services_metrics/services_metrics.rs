use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::MetricEvent;

pub struct ServiesMetrics {
    pub metrics: BTreeMap<String, BTreeMap<i64, Vec<MetricEvent>>>,
}

impl ServiesMetrics {
    pub fn new() -> Self {
        Self {
            metrics: BTreeMap::new(),
        }
    }

    pub fn new_event(
        &mut self,
        service_name: String,
        event_data: String,
        started: i64,
        finished: i64,
        process_id: i64,
        success: Option<String>,
        fail: Option<String>,
    ) {
        if !self.metrics.contains_key(service_name.as_str()) {
            self.metrics.insert(service_name.clone(), BTreeMap::new());
        }

        let metrics_by_app = self.metrics.get_mut(service_name.as_str()).unwrap();

        if !metrics_by_app.contains_key(&process_id) {
            metrics_by_app.insert(process_id, Vec::new());
        }

        metrics_by_app
            .get_mut(&process_id)
            .unwrap()
            .push(MetricEvent {
                started: DateTimeAsMicroseconds::new(started),
                finished: DateTimeAsMicroseconds::new(finished),
                service_name,
                event_data,
                success,
                fail,
            });
    }

    pub fn gc(&mut self) {
        for items in self.metrics.values_mut() {
            for sub_items in items.values_mut() {
                while sub_items.len() > 1000 {
                    sub_items.remove(0);
                }
            }
        }
    }

    pub fn get_services(&self) -> Vec<String> {
        self.metrics.keys().cloned().collect()
    }
}
