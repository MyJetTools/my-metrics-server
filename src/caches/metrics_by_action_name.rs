use std::collections::{BTreeMap, HashMap};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::postgres::dto::MetricDto;

use super::{MetricByHour, MetricsByHour};

pub struct MetricsByActionName {
    data: HashMap<String, MetricsByHour>,
}

impl MetricsByActionName {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn restore(&mut self, data: &str, hour: i64, metric_by_hour: MetricByHour) {
        if !self.data.contains_key(data) {
            self.data.insert(data.to_string(), MetricsByHour::new());
        }
        self.data
            .get_mut(data)
            .unwrap()
            .restore(hour, metric_by_hour);
    }

    pub fn get_to_update(&mut self, event: &MetricDto) -> Option<&mut MetricByHour> {
        let service_data = self.data.get_mut(&event.data)?;
        service_data.get_to_update(event)
    }

    pub fn get_metrics_to_save(&self) -> Option<BTreeMap<String, BTreeMap<i64, MetricByHour>>> {
        let mut result = None;

        for (action_name, data) in self.data.iter() {
            if let Some(to_save) = data.get_metrics_to_save() {
                if result.is_none() {
                    result = Some(BTreeMap::new());
                }

                result
                    .as_mut()
                    .unwrap()
                    .insert(action_name.to_string(), to_save);
            }
        }

        result
    }

    pub fn confirm_metrics_saved(&mut self, data: &BTreeMap<String, BTreeMap<i64, MetricByHour>>) {
        for (action_name, data) in data {
            if let Some(data_to_confirm) = self.data.get_mut(action_name) {
                data_to_confirm.confirm_metrics_saved(data);
            }
        }
    }

    pub fn gc_old_data(&mut self, now: DateTimeAsMicroseconds) -> usize {
        let mut to_gc = Vec::new();
        for (action_name, data) in self.data.iter_mut() {
            let amount = data.gc_old_data(now);
            if amount == 0 {
                to_gc.push(action_name.to_string());
            }
        }

        for key in to_gc {
            self.data.remove(&key);
        }

        self.data.len()
    }
}
