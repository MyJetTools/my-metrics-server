use std::collections::BTreeMap;

use rust_extensions::{
    date_time::{HourKey, IntervalKey},
    InsertOrUpdateEntry2Keys, SortedVecWith2StrKey,
};

use crate::db::*;

use super::AppDataHourStatistics;

pub struct StatisticsByHourAndServiceName {
    data: BTreeMap<i64, SortedVecWith2StrKey<AppDataHourStatistics>>,
    to_persist: BTreeMap<i64, SortedVecWith2StrKey<AppDataHourStatistics>>,
}

impl StatisticsByHourAndServiceName {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            to_persist: BTreeMap::new(),
        }
    }

    pub fn update(&mut self, hour_key: IntervalKey<HourKey>, events: &[MetricDto]) {
        if !self.data.contains_key(hour_key.as_i64_ref()) {
            self.data
                .insert(hour_key.to_i64(), SortedVecWith2StrKey::new());
        }

        let sub_items = self.data.get_mut(hour_key.as_i64_ref()).unwrap();

        let mut to_persist = Vec::new();

        for itm in events {
            match sub_items.insert_or_update(&itm.name, &itm.data) {
                InsertOrUpdateEntry2Keys::Insert(entry) => {
                    let itm = AppDataHourStatistics::new(itm);
                    entry.insert(itm.clone());
                    to_persist.push(itm);
                }
                InsertOrUpdateEntry2Keys::Update(mut entry) => {
                    let itm_to_persist = entry.get_item_mut().update(itm);
                    to_persist.push(itm_to_persist);
                }
            }
        }

        self.set_to_persist(hour_key, to_persist);
    }

    fn set_to_persist(
        &mut self,
        hour_key: IntervalKey<HourKey>,
        events_to_persist: Vec<AppDataHourStatistics>,
    ) {
        if !self.to_persist.contains_key(hour_key.as_i64_ref()) {
            self.to_persist
                .insert(hour_key.to_i64(), SortedVecWith2StrKey::new());
        }

        let sub_items = self.to_persist.get_mut(hour_key.as_i64_ref()).unwrap();

        for itm in events_to_persist {
            match sub_items.insert_or_update(&itm.service, &itm.data) {
                InsertOrUpdateEntry2Keys::Insert(entry) => {
                    entry.insert(itm);
                }
                InsertOrUpdateEntry2Keys::Update(mut entry) => {
                    entry.get_item_mut().update_to_persist(&itm);
                }
            }
        }
    }

    pub fn restore(
        &mut self,
        hour_key: IntervalKey<HourKey>,
        metric_by_hour: Vec<HourAppDataStatisticsDto>,
    ) {
        if !self.data.contains_key(hour_key.as_i64_ref()) {
            self.data
                .insert(hour_key.to_i64(), SortedVecWith2StrKey::new());
        }

        let sub_items = self.data.get_mut(hour_key.as_i64_ref()).unwrap();

        for itm in metric_by_hour {
            let itm: AppDataHourStatistics = itm.into();
            sub_items.insert_or_replace(itm.into());
        }
    }

    pub fn get_to_persist(
        &mut self,
    ) -> Option<BTreeMap<i64, SortedVecWith2StrKey<AppDataHourStatistics>>> {
        if self.to_persist.is_empty() {
            return None;
        }
        let mut result = BTreeMap::new();

        std::mem::swap(&mut result, &mut self.to_persist);

        Some(result)
    }

    pub fn get<TResult>(
        &self,
        hour_key: IntervalKey<HourKey>,
        app: &str,
        convert: impl Fn(&AppDataHourStatistics) -> TResult,
    ) -> Option<Vec<TResult>> {
        let items = self.data.get(hour_key.as_i64_ref())?;

        let sub_items = items.get_by_primary_key(app)?;

        let mut result = Vec::with_capacity(self.data.len());

        for itm in sub_items {
            result.push(convert(itm));
        }

        Some(result)
    }

    pub fn gc_old_data(&mut self, from_hour: IntervalKey<HourKey>) {
        let to_gc = {
            let mut to_gc = Vec::new();
            for hour_key in self.data.keys() {
                let hour_key = *hour_key;
                if hour_key <= from_hour.to_i64() {
                    to_gc.push(hour_key);
                }
            }

            to_gc
        };

        if !to_gc.is_empty() {
            println!("GC hour_service_name {:?}", to_gc);
        }

        for hour_key in to_gc {
            self.data.remove(&hour_key);
        }
    }
}
