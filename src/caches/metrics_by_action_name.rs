use std::collections::HashMap;

use crate::postgres::dto::MetricDto;

use super::{MetricsByHour, ServiceInfo};

pub struct MetricsByActionName {
    data: HashMap<String, MetricsByHour>,
}

impl MetricsByActionName {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn update(&mut self, event: &MetricDto) {
        if !self.data.contains_key(&event.data) {
            self.data
                .insert(event.data.to_string(), MetricsByHour::new());
        }

        self.data.get_mut(&event.data).unwrap().update(event);
    }

    pub fn get_avg(&self) -> ServiceInfo {
        let mut avg_result = 0;

        let mut amount = 0;

        let mut total_amount = 0;

        for itm in self.data.values() {
            let avg = itm.get_avg_value();
            avg_result += avg.avg;
            amount += 1;
            total_amount += avg.amount;
        }

        ServiceInfo {
            avg: avg_result / amount,
            amount: total_amount,
        }
    }
}
