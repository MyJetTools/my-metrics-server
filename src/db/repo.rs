use std::collections::BTreeMap;

use rust_extensions::{
    date_time::{DateTimeAsMicroseconds, HourKey},
    StopWatch,
};

use super::{dto::*, SqlLitePool};

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

    pub async fn insert(&self, dto_s: Vec<MetricDto>) -> BTreeMap<HourKey, Vec<MetricDto>> {
        let mut by_hour_key = BTreeMap::new();

        for dto in dto_s {
            let hour_key: HourKey = dto.started.into();

            if !by_hour_key.contains_key(&hour_key) {
                by_hour_key.insert(hour_key, Vec::new());
            }

            by_hour_key.get_mut(&hour_key).unwrap().push(dto);
        }

        for (hour_key, items) in &by_hour_key {
            println!("Inserting {} metrics for hour: {:?}", items.len(), hour_key);
            let connection = self.pool.get_for_write_access(*hour_key).await;

            let result = connection
                .bulk_insert_db_entities_if_not_exists(&items, TABLE_NAME)
                .await;

            if let Err(err) = result {
                println!("Failed to write metrics to db: {:?}", err);
            }
        }

        by_hour_key
    }
    pub async fn get_by_process_id(&self, process_id: i64) -> Vec<MetricDto> {
        let where_model = WhereByProcessId { id: process_id };
        let mut sw = StopWatch::new();
        sw.start();

        let result = if let Some(last) = self.pool.get_last().await {
            last.query_rows(TABLE_NAME, Some(&where_model))
                .await
                .unwrap()
        } else {
            vec![]
        };

        sw.pause();

        println!("get_by_process_id finished in: {:?}", sw.duration());

        result
    }

    pub async fn get_by_service_name(&self, service_name: &str, data: &str) -> Vec<MetricDto> {
        let where_model = WhereByServiceName {
            name: service_name,
            data,
            limit: 100,
        };

        let mut sw = StopWatch::new();
        sw.start();

        let result = if let Some(last) = self.pool.get_last().await {
            last.query_rows(TABLE_NAME, Some(&where_model))
                .await
                .unwrap()
        } else {
            vec![]
        };

        sw.pause();

        println!("get_by_service_name finished in: {:?}", sw.duration());

        result
    }

    /*
          pub async fn get_services(&self, from: DateTimeAsMicroseconds) -> Vec<ServiceDto> {
              let where_model = FromStartedWhereModel {
                  started: from.unix_microseconds,
              };

              let mut sw = StopWatch::new();
              sw.start();

              let result = self
                  .postgres_read
                  .query_rows(TABLE_NAME, Some(&where_model))
                  .await
                  .unwrap();

              sw.pause();

              println!("get_services finished in: {:?}", sw.duration());

              result
          }


       pub async fn get_service_overview(
           &self,
           service_name: &str,
           from: DateTimeAsMicroseconds,
       ) -> Vec<ServiceOverviewDto> {
           let where_model = FromStartedAndServiceNameWhereModel {
               started: from.unix_microseconds,
               name: service_name,
           };

           let mut sw = StopWatch::new();
           sw.start();

           let metrics: Vec<MetricDto> = self
               .postgres_read
               .query_rows(TABLE_NAME, Some(&where_model))
               .await
               .unwrap();

           sw.pause();

           println!("get_service_overview finished in: {:?}", sw.duration());

           ServiceOverviewDto::from_metric_dto(metrics)
       }

       pub async fn get_events_amount(&self) -> usize {
           let result: Option<usize> = self
               .postgres_read
               .get_count(TABLE_NAME, NoneWhereModel::new())
               .await
               .unwrap();

           if result.is_none() {
               return 0;
           }

           result.unwrap()
       }
    */
    pub async fn gc(&self, from: DateTimeAsMicroseconds) {
        self.pool.gc(from).await;
    }
}
