use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::MetricEvent;

pub struct ServiceDomainModel {
    pub id: String,
    pub avg: i64,
}

pub struct ServiceOverviewDomainModel {
    pub data: String,
    pub min: i64,
    pub max: i64,
    pub avg: i64,
    pub success: usize,
    pub error: usize,
    pub total: usize,
}

pub struct ServiesMetrics {
    pub metrics: BTreeMap<String, BTreeMap<i64, Vec<Arc<MetricEvent>>>>,
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
        ip: Option<String>,
    ) {
        if !self.metrics.contains_key(service_name.as_str()) {
            self.metrics.insert(service_name.clone(), BTreeMap::new());
        }

        let metrics_by_app = self.metrics.get_mut(service_name.as_str()).unwrap();

        if !metrics_by_app.contains_key(&process_id) {
            metrics_by_app.insert(process_id, Vec::new());
        }

        metrics_by_app.get_mut(&process_id).unwrap().push(
            MetricEvent {
                started: DateTimeAsMicroseconds::new(started),
                finished: DateTimeAsMicroseconds::new(finished),
                service_name,
                event_data,
                success,
                fail,
                ip,
            }
            .into(),
        );
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

    pub fn get_services(&self) -> Vec<ServiceDomainModel> {
        let mut result = Vec::with_capacity(self.metrics.len());
        for (id, services) in &self.metrics {
            result.push(ServiceDomainModel {
                id: id.clone(),
                avg: get_avg_duration(services),
            });
        }

        result
    }

    pub fn get_metrics_by_resource(
        &self,
        service_id: &str,
        resource_data: &str,
    ) -> Vec<Arc<MetricEvent>> {
        let mut result = Vec::new();
        if let Some(service) = self.metrics.get(service_id) {
            for (_, events) in service {
                for event in events {
                    if event.event_data == resource_data {
                        result.push(event.clone());
                    }
                }
            }
        }

        result
    }

    pub fn get_service_overview(
        &self,
        service_id: &str,
    ) -> HashMap<String, ServiceOverviewDomainModel> {
        let mut result = HashMap::new();

        if let Some(service) = self.metrics.get(service_id) {
            for (_, events) in service {
                for event in events {
                    if !result.contains_key(&event.event_data) {
                        let duration = event.get_duration_mcs();

                        let mut success = 0;

                        if event.is_success() {
                            success = 1;
                        }

                        let mut fail = 0;

                        if event.is_fail() {
                            fail = 1;
                        }

                        result.insert(
                            event.event_data.to_string(),
                            ServiceOverviewDomainModel {
                                data: event.event_data.to_string(),
                                min: duration,
                                max: duration,
                                avg: duration,
                                success: success,
                                error: fail,
                                total: 1,
                            },
                        );
                    } else {
                        if let Some(event_metric) = result.get_mut(&event.event_data) {
                            let duration = event.get_duration_mcs();

                            if duration < event_metric.min {
                                event_metric.min = duration;
                            }

                            if duration > event_metric.max {
                                event_metric.max = duration;
                            }

                            if duration > event_metric.max {
                                event_metric.avg += duration;
                            }

                            if event.is_success() {
                                event_metric.success += 1;
                            }

                            if event.is_fail() {
                                event_metric.error += 1;
                            }

                            event_metric.total += 1;
                        }
                    }
                }
            }
        }

        for event_metrics in result.values_mut() {
            event_metrics.avg = event_metrics.avg / event_metrics.total as i64;
        }

        result
    }
}

fn get_avg_duration(src: &BTreeMap<i64, Vec<Arc<MetricEvent>>>) -> i64 {
    let mut sum = 0;
    let mut amount = 0;

    for events in src.values() {
        for event in events {
            sum += event.get_duration_mcs();
            amount += 1;
        }
    }

    sum / amount
}
