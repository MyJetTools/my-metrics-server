use std::collections::BTreeMap;

use crate::postgres::dto::MetricDto;

use super::{ActionInfo, ServiceInfo};

pub struct MetricByHour {
    pub min: i64,
    pub max: i64,
    pub errors_amount: i64,
    pub success_amount: i64,
    pub sum_of_duration: i64,
    pub amount: i64,
}

impl MetricByHour {
    pub fn new(src: &MetricDto) -> Self {
        Self {
            min: src.duration_micro,
            max: src.duration_micro,
            sum_of_duration: src.duration_micro,
            amount: 1,
            success_amount: if src.success.is_some() { 1 } else { 0 },
            errors_amount: if src.fail.is_some() { 1 } else { 0 },
        }
    }

    pub fn update(&mut self, itm: &MetricDto) {
        if itm.duration_micro < self.min {
            self.min = itm.duration_micro;
        }

        if itm.duration_micro < self.max {
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

    fn gc(&mut self) {
        while self.data.len() > 24 {
            let first = *self.data.first_key_value().unwrap().0;
            self.data.remove(&first);
        }
    }
    pub fn update(&mut self, event: &MetricDto) {
        let rounded_by_hour = round_by_hour(event.started);

        if !self.data.contains_key(&rounded_by_hour) {
            self.data.insert(rounded_by_hour, MetricByHour::new(event));
            return;
        }

        self.data.get_mut(&rounded_by_hour).unwrap().update(event);

        self.gc()
    }

    pub fn get_avg_value(&self) -> ServiceInfo {
        let mut avg_result = 0;

        let mut amount = 0;

        let mut total_amount = 0;

        for itm in self.data.values() {
            avg_result += itm.sum_of_duration / itm.amount;
            total_amount += itm.amount;
            amount += 1;
        }
        ServiceInfo {
            avg: avg_result / amount,
            amount: total_amount,
        }
    }

    pub fn get_action_info(&self) -> ActionInfo {
        let mut min = None;
        let mut max = None;

        let mut sum_of_duration = 0;
        let mut total_amount = 0;

        let mut success = 0;
        let mut errors = 0;
        for itm in self.data.values() {
            match &mut min {
                Some(value) => {
                    if *value > itm.min {
                        *value = itm.min
                    }
                }
                None => {
                    min = Some(itm.min);
                }
            }

            match &mut max {
                Some(value) => {
                    if *value < itm.max {
                        *value = itm.max
                    }
                }
                None => {
                    max = Some(itm.max);
                }
            }

            sum_of_duration += itm.sum_of_duration;
            total_amount += itm.amount;
            success += itm.success_amount;
            errors += itm.errors_amount;
        }

        ActionInfo {
            max: if let Some(max) = max { max } else { 0 },
            min: if let Some(min) = min { min } else { 0 },
            avg: sum_of_duration / total_amount,
            success,
            errors,
        }
    }
}

fn round_by_hour(micro_seconds: i64) -> i64 {
    micro_seconds - micro_seconds % 3600_000_000
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::round_by_hour;

    #[test]
    fn test_round_by_hour() {
        let dt = DateTimeAsMicroseconds::from_str("2015-01-05:12:43.23.123").unwrap();

        let rounded = round_by_hour(dt.unix_microseconds);

        let dest = DateTimeAsMicroseconds::new(rounded);

        assert_eq!(&dest.to_rfc3339()[..19], "2015-01-05T12:00:00");
    }
}
