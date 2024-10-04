use my_sqlite::{SqlLiteConnection, SqlLiteConnectionBuilder};
use rust_extensions::{date_time::DateTimeAsMicroseconds, StopWatch};

use super::dto::*;

const TABLE_NAME: &'static str = "metrics";

//const PK_NAME: &'static str = "metrics_pk";

pub struct MetricsPostgresRepo {
    connection: SqlLiteConnection,
}

impl MetricsPostgresRepo {
    pub async fn new(file_name: String) -> Self {
        Self {
            connection: SqlLiteConnectionBuilder::new(file_name)
                .create_table_if_no_exists::<MetricDto>(TABLE_NAME)
                .build()
                .await
                .unwrap(),
        }
    }

    pub async fn insert(&self, dto_s: &[MetricDto]) {
        let result = self
            .connection
            .bulk_insert_db_entities_if_not_exists(dto_s, TABLE_NAME)
            .await;

        if let Err(err) = result {
            println!("Failed to write metrics to postgres: {:?}", err);
        }
    }
    pub async fn get_by_process_id(&self, process_id: i64) -> Vec<MetricDto> {
        let where_model = WhereByProcessId { id: process_id };
        let mut sw = StopWatch::new();
        sw.start();
        let result = self
            .connection
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap();

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

        let result = self
            .connection
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap();

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
        let where_model = GcWhereModel {
            id: from.unix_microseconds,
        };

        self.connection
            .delete_db_entity(TABLE_NAME, &where_model)
            .await
            .unwrap();
    }
}
