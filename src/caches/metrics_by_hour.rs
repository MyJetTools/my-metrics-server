use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::postgres::dto::{MetricDto, StatisticsDto};

#[derive(Clone)]
pub struct MetricByHour {
    pub min: i64,
    pub max: i64,
    pub errors_amount: i64,
    pub success_amount: i64,
    pub sum_of_duration: i64,
    pub amount: i64,
    pub persist_me: bool,
}

impl Into<MetricByHour> for StatisticsDto {
    fn into(self) -> MetricByHour {
        MetricByHour {
            min: self.min,
            max: self.max,
            errors_amount: self.errors_amount,
            success_amount: self.success_amount,
            sum_of_duration: self.sum_of_duration,
            amount: self.amount,
            persist_me: false,
        }
    }
}

impl MetricByHour {
    pub fn update(&mut self, itm: &MetricDto) {
        if itm.duration_micro < self.min || self.min == 0 {
            self.min = itm.duration_micro;
        }

        if itm.duration_micro > self.max {
            self.max = itm.duration_micro;
        }

        self.sum_of_duration += itm.duration_micro;
        self.amount += 1;

        if itm.success.is_some() {
            self.success_amount += 1;
        }

        if itm.fail.is_some() {
            self.errors_amount += 1;
        }

        self.persist_me = true;
    }
}

pub struct MetricsByHour {
    data: BTreeMap<i64, MetricByHour>,
}

impl MetricsByHour {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn restore(&mut self, hour: i64, metric: MetricByHour) {
        self.data.insert(hour, metric);
    }

    pub fn get_to_update(&mut self, event: &MetricDto) -> Option<&mut MetricByHour> {
        let rounded_by_hour = event.get_rounded_hour();
        self.data.get_mut(&rounded_by_hour)
    }

    pub fn get_metrics_to_save(&self) -> Option<BTreeMap<i64, MetricByHour>> {
        let mut result: Option<BTreeMap<i64, MetricByHour>> = None;

        for (hour, data) in self.data.iter() {
            if data.persist_me {
                if result.is_none() {
                    result = Some(BTreeMap::new());
                }

                result.as_mut().unwrap().insert(*hour, data.clone());
            }
        }

        result
    }

    pub fn confirm_metrics_saved(&mut self, data: &BTreeMap<i64, MetricByHour>) {
        for hour in data.keys() {
            if let Some(data_to_confirm) = self.data.get_mut(hour) {
                data_to_confirm.persist_me = false;
            }
        }
    }

    pub fn gc_old_data(&mut self, mut now: DateTimeAsMicroseconds) -> usize {
        now.add_hours(-2);

        let mut keys_to_gc = Vec::new();

        {
            for key in self.data.keys() {
                if key < &now.unix_microseconds {
                    keys_to_gc.push(*key);
                }
            }
        }

        for key_to_gc in keys_to_gc {
            self.data.remove(&key_to_gc);
        }

        self.data.len()
    }
}
