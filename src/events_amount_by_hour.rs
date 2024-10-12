use std::collections::BTreeMap;

use rust_extensions::{
    date_time::{HourKey, IntervalKey},
    sorted_vec::{EntityWithStrKey, InsertOrUpdateEntry, SortedVecWithStrKey},
};

use crate::db::*;

#[derive(Debug, Clone)]
pub struct StatisticsByHour {
    pub name: String,
    pub amount: i64,
    pub duration_micros: i64,
}

impl EntityWithStrKey for StatisticsByHour {
    fn get_key(&self) -> &str {
        self.name.as_str()
    }
}

impl StatisticsByHour {
    pub fn new(name: String, duration_micros: i64) -> Self {
        Self {
            name,
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

impl Into<StatisticsByHour> for HourStatisticsDto {
    fn into(self) -> StatisticsByHour {
        StatisticsByHour {
            name: self.app,
            amount: self.amount,
            duration_micros: self.duration_micros,
        }
    }
}

pub struct EventAmountsByHour {
    pub items: BTreeMap<i64, SortedVecWithStrKey<StatisticsByHour>>,
    pub to_persist: BTreeMap<i64, SortedVecWithStrKey<StatisticsByHour>>,
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
            /*
                       Some(count) => count.inc(metric_dto.duration_micro),
                       None => {
                           let item = ;
                           items.insert(metric_dto.name.to_string(), item);
                           item
                       }

            */
            match items.insert_or_update(metric_dto.name.as_str()) {
                InsertOrUpdateEntry::Insert(entry) => {
                    let item = StatisticsByHour::new(
                        metric_dto.name.to_string(),
                        metric_dto.duration_micro,
                    );
                    entry.insert(item.clone());
                    item
                }
                InsertOrUpdateEntry::Update(update_entry) => {
                    update_entry.item.inc(metric_dto.duration_micro);
                    update_entry.item.clone()
                }
            }
        } else {
            let mut sorted_vec = SortedVecWithStrKey::new();
            let item =
                StatisticsByHour::new(metric_dto.name.to_string(), metric_dto.duration_micro);
            sorted_vec.insert_or_replace(item.clone());
            self.items.insert(as_i64, sorted_vec);
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
                    sub_items.insert_or_replace(to_persist);
                }
            }

            return;
        }

        let mut sub_items = SortedVecWithStrKey::new();
        sub_items.insert_or_replace(to_persist);
        self.to_persist.insert(interval_key, sub_items);
    }

    pub fn get_to_persist(
        &mut self,
    ) -> Option<BTreeMap<i64, SortedVecWithStrKey<StatisticsByHour>>> {
        if self.to_persist.is_empty() {
            return None;
        }

        let mut result = BTreeMap::new();

        std::mem::swap(&mut self.to_persist, &mut result);

        Some(result)
    }

    pub fn restore(
        &mut self,
        hour_key: IntervalKey<HourKey>,
        items_to_restore: impl Iterator<Item = StatisticsByHour>,
    ) {
        let mut result = SortedVecWithStrKey::new();

        for itm in items_to_restore {
            result.insert_or_replace(itm);
        }
        self.items.insert(hour_key.to_i64(), result);
    }

    pub fn get<TResult>(
        &self,
        hour_key: IntervalKey<HourKey>,
        convert: impl Fn(&StatisticsByHour) -> TResult,
    ) -> Option<Vec<TResult>> {
        let items = self.items.get(hour_key.as_i64_ref())?;

        let mut result = Vec::with_capacity(self.items.len());

        for itm in items.iter() {
            result.push(convert(itm));
        }

        Some(result)
    }

    pub fn gc_old_data(&mut self, from_hour: IntervalKey<HourKey>) {
        let to_gc = {
            let mut to_gc = Vec::new();
            for hour_key in self.items.keys() {
                let hour_key = *hour_key;
                if hour_key <= from_hour.to_i64() {
                    to_gc.push(hour_key);
                }
            }

            to_gc
        };

        if !to_gc.is_empty() {
            println!("GC amounts_by_hour {:?}", to_gc);
        }

        for hour_key in to_gc {
            self.items.remove(&hour_key);
        }
    }
}
