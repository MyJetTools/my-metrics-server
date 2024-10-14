use std::collections::BTreeMap;

use rust_extensions::{
    date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey},
    StopWatch,
};

use crate::db::metrics::dto::*;

use super::SqlLitePool;

pub const TABLE_NAME: &'static str = "metrics";

//const PK_NAME: &'static str = "metrics_pk";

pub struct MetricsRepo {
    pool: SqlLitePool,
}

impl MetricsRepo {
    pub async fn new(file_name: String) -> Self {
        Self {
            pool: SqlLitePool::new(file_name),
        }
    }

    pub async fn insert(
        &self,
        dto_s: Vec<MetricDto>,
    ) -> BTreeMap<IntervalKey<HourKey>, Vec<MetricDto>> {
        let mut by_hour_key = BTreeMap::new();

        for dto in dto_s {
            let dt = DateTimeAsMicroseconds::from(dto.started);
            let hour_key: IntervalKey<HourKey> = dt.into();

            if !by_hour_key.contains_key(&hour_key) {
                by_hour_key.insert(hour_key, Vec::new());
            }

            by_hour_key.get_mut(&hour_key).unwrap().push(dto);
        }

        for (hour_key, items) in &by_hour_key {
            // println!("Inserting {} metrics for hour: {:?}", items.len(), hour_key);
            let connection = self.pool.get_for_write_access(*hour_key).await;

            let connection = connection.lock().await;
            let result = connection
                .bulk_insert_db_entities_if_not_exists(&items, TABLE_NAME)
                .await;

            if let Err(err) = result {
                println!("Failed to write metrics to db: {:?}", err);
            }
        }

        by_hour_key
    }
    pub async fn get_by_process_id(
        &self,
        hour_key: IntervalKey<HourKey>,
        process_id: i64,
    ) -> Vec<MetricDto> {
        println!("Requested get_by_process_id process_id: {}", process_id);
        let where_model = WhereByProcessId { id: process_id };
        let mut sw = StopWatch::new();
        sw.start();

        let result = if let Some(last) = self.pool.get_for_read_access(hour_key).await {
            let connection = last.lock().await;
            connection
                .query_rows(TABLE_NAME, Some(&where_model))
                .await
                .unwrap()
        } else {
            vec![]
        };

        sw.pause();

        println!("get_by_process_id finished in: {:?}", sw.duration());

        result
    }

    pub async fn get_by_service_name(
        &self,
        hour_key: IntervalKey<HourKey>,
        service_name: &str,
        data: &str,
    ) -> Vec<MetricDto> {
        println!(
            "Requested get_by_service_name for: {} with data: {}",
            service_name, data
        );
        let where_model = WhereByServiceName {
            name: service_name,
            data,
            limit: 100,
        };

        let mut sw = StopWatch::new();
        sw.start();

        let result = if let Some(connection) = self.pool.get_for_read_access(hour_key).await {
            let connection = connection.lock().await;
            connection
                .query_rows(TABLE_NAME, Some(&where_model))
                .await
                .unwrap()
        } else {
            vec![]
        };

        sw.pause();

        println!("get_by_service_name finished in: {:?}", sw.duration());

        result
    }

    pub async fn gc(&self, hour_key: IntervalKey<HourKey>) {
        self.pool.gc_file(hour_key).await;
    }
}
