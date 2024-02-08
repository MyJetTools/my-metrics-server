use std::{sync::Arc, time::Duration};

use my_postgres::{sql_where::NoneWhereModel, MyPostgres, PostgresSettings};
use rust_extensions::{date_time::DateTimeAsMicroseconds, StopWatch};

use crate::app_ctx::APP_NAME;

use super::dto::*;

const TABLE_NAME: &'static str = "metrics";

const PK_NAME: &'static str = "metrics_pk";

pub struct MetricsPostgresRepo {
    postgres_write: MyPostgres,
    postgres_read: MyPostgres,
    postgres_gc: MyPostgres,
}

impl MetricsPostgresRepo {
    pub async fn new(postgres_settings: Arc<dyn PostgresSettings + Sync + Send + 'static>) -> Self {
        Self {
            postgres_write: MyPostgres::from_settings(APP_NAME, postgres_settings.clone())
                .set_sql_request_timeout(Duration::from_secs(60))
                .with_table_schema_verification::<MetricDto>(TABLE_NAME, Some(PK_NAME.into()))
                .build()
                .await,

            postgres_read: MyPostgres::from_settings(APP_NAME, postgres_settings.clone())
                .set_sql_request_timeout(Duration::from_secs(60))
                .build()
                .await,

            postgres_gc: MyPostgres::from_settings(APP_NAME, postgres_settings.clone())
                .set_sql_request_timeout(Duration::from_secs(60))
                .build()
                .await,
        }
    }

    pub async fn insert(&self, dto_s: &[MetricDto]) {
        let result = self
            .postgres_write
            .bulk_insert_db_entities_if_not_exists(TABLE_NAME, dto_s)
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
            .postgres_read
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
            .postgres_read
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
    */
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

    pub async fn gc(&self, from: DateTimeAsMicroseconds) {
        let where_model = GcWhereModel {
            id: from.unix_microseconds,
        };

        self.postgres_gc
            .with_retries(3, Duration::from_secs(3))
            .delete_db_entity(TABLE_NAME, &where_model)
            .await
            .unwrap();
    }
}
