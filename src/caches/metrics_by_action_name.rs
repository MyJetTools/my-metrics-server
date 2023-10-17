use std::collections::{BTreeMap, HashMap};

use crate::postgres::dto::MetricDto;

use super::{ActionInfo, MetricsByHour, ServiceInfo};

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

    pub fn get_action_info(&self) -> BTreeMap<String, ActionInfo> {
        let mut result = BTreeMap::new();

        for (action_name, data) in self.data.iter() {
            result.insert(action_name.to_string(), data.get_action_info());
        }

        result
    }
}
