use std::collections::BTreeMap;

use crate::postgres::dto::MetricDto;

pub struct MetricByHour {
    pub min: i64,
    pub max: i64,
    pub amount: i64,
}

impl MetricByHour {
    pub fn new(src: &MetricDto) -> Self {
        Self {
            min: src.duration_micro,
            max: src.duration_micro,
            amount: 1,
        }
    }

    pub fn update(&mut self, itm: &MetricDto) {
        if itm.duration_micro < self.min {
            self.min = itm.duration_micro;
        }

        if itm.duration_micro < self.max {
            self.max = itm.duration_micro;
        }

        self.amount += 1;
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

    pub fn get_avg_value(&self) -> i64 {
        let mut avg_result = 0;

        let mut amount = 0;

        for itm in self.data.values() {
            avg_result += itm.max - itm.min;
            amount += 1;
        }

        avg_result / amount
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
