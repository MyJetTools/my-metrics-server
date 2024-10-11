use std::collections::{BTreeMap, HashMap};

use rust_extensions::date_time::{HourKey, IntervalKey};

use crate::db::MetricDto;

#[derive(Debug, Clone, Copy)]
pub struct StatisticsByHour {
    pub amount: i64,
    pub duration_micros: i64,
}

impl StatisticsByHour {
    pub fn new(duration_micros: i64) -> Self {
        Self {
            amount: 1,
            duration_micros,
        }
    }

    pub fn update(&mut self, other: Self) {
        *self = other;
    }
    pub fn inc(&mut self, duration_micros: i64) -> StatisticsByHour {
        self.amount += 1;
        self.duration_micros += duration_micros;

        self.clone()
    }
}

pub struct EventAmountsByHour {
    pub items: BTreeMap<i64, HashMap<String, StatisticsByHour>>,
    pub to_persist: BTreeMap<i64, HashMap<String, StatisticsByHour>>,
}

impl EventAmountsByHour {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
            to_persist: BTreeMap::new(),
        }
    }

    pub fn inc(&mut self, interval_key: IntervalKey<HourKey>, metric_dto: &MetricDto) {
        let as_i64 = interval_key.to_i64();

        let to_persist = if let Some(items) = self.items.get_mut(&as_i64) {
            match items.get_mut(metric_dto.name.as_str()) {
                Some(count) => count.inc(metric_dto.duration_micro),
                None => {
                    let item = StatisticsByHour::new(metric_dto.duration_micro);
                    items.insert(metric_dto.name.to_string(), item);
                    item
                }
            }
        } else {
            let item = StatisticsByHour::new(metric_dto.duration_micro);
            let mut sub_items = HashMap::new();
            sub_items.insert(metric_dto.name.to_string(), item);
            self.items.insert(as_i64, sub_items);
            item
        };

        self.engage_persist(as_i64, metric_dto.name.as_str(), to_persist);
    }

    fn engage_persist(&mut self, interval_key: i64, app: &str, to_persist: StatisticsByHour) {
        if let Some(sub_items) = self.to_persist.get_mut(&interval_key) {
            match sub_items.get_mut(app) {
                Some(count) => {
                    count.update(to_persist);
                }
                None => {
                    sub_items.insert(app.to_string(), to_persist);
                }
            }

            return;
        }

        let mut sub_items = HashMap::new();
        sub_items.insert(app.to_string(), to_persist);
        self.to_persist.insert(interval_key, sub_items);
    }

    pub fn get_to_persist(&mut self) -> Option<BTreeMap<i64, HashMap<String, StatisticsByHour>>> {
        if self.to_persist.is_empty() {
            return None;
        }

        let mut result = BTreeMap::new();

        std::mem::swap(&mut self.to_persist, &mut result);

        Some(result)
    }
}
