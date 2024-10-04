use std::collections::{BTreeMap, HashMap};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::db::dto::MetricDto;

use super::{MetricByHour, MetricsByActionName};

pub struct AggregatedMetricsByServiceCache {
    data: HashMap<String, MetricsByActionName>,
}

impl AggregatedMetricsByServiceCache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get_to_update(&mut self, event: &MetricDto) -> Option<&mut MetricByHour> {
        let service_data = self.data.get_mut(&event.name)?;
        service_data.get_to_update(event)
    }

    pub fn restore(&mut self, service: &str, data: &str, hour: i64, metric_by_hour: MetricByHour) {
        if !self.data.contains_key(service) {
            self.data
                .insert(service.to_string(), MetricsByActionName::new());
        }

        self.data
            .get_mut(service)
            .unwrap()
            .restore(data, hour, metric_by_hour);
    }

    /*
       pub fn get_services(&self) -> BTreeMap<String, ServiceInfo> {
           let mut result = BTreeMap::new();

           for (service_name, data) in self.data.iter() {
               result.insert(service_name.to_string(), data.get_avg());
           }

           result
       }

       pub fn get_actions_statistics(
           &self,
           service_name: &str,
       ) -> Option<BTreeMap<String, ActionInfo>> {
           if let Some(data) = self.data.get(service_name) {
               return Some(data.get_action_info());
           }

           None
       }
    */

    pub fn get_metrics_to_save(
        &self,
    ) -> Option<BTreeMap<String, BTreeMap<String, BTreeMap<i64, MetricByHour>>>> {
        let mut result = None;

        for (service_name, data) in self.data.iter() {
            if let Some(to_save) = data.get_metrics_to_save() {
                if result.is_none() {
                    result = Some(BTreeMap::new());
                }
                result
                    .as_mut()
                    .unwrap()
                    .insert(service_name.to_string(), to_save);
            }
        }

        result
    }

    pub fn confirm_metrics_saved(
        &mut self,
        metrics: &BTreeMap<String, BTreeMap<String, BTreeMap<i64, MetricByHour>>>,
    ) {
        for (service_name, data) in metrics {
            if let Some(data_from_dict) = self.data.get_mut(service_name) {
                data_from_dict.confirm_metrics_saved(data);
            }
        }

        self.gc_old_data(DateTimeAsMicroseconds::now());
    }

    fn gc_old_data(&mut self, now: DateTimeAsMicroseconds) {
        let to_gc = {
            let mut to_gc = Vec::new();
            for (service_id, data) in self.data.iter_mut() {
                let amount_after_gc = data.gc_old_data(now);

                if amount_after_gc == 0 {
                    to_gc.push(service_id.clone());
                }
            }

            to_gc
        };

        for key in &to_gc {
            self.data.remove(key);
        }
    }
}
