use std::{sync::Arc, time::Duration};

use my_postgres::{MyPostgres, PostgresSettings, UpdateConflictType};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::APP_NAME;

use super::dto::*;

const TABLE_NAME: &'static str = "statistics";

const PK_NAME: &'static str = "statistics_pk";

#[derive(Debug)]
pub struct AggregatedStatistics {
    pub service: String,
    pub avg: i64,
    pub amount: i64,
}

#[derive(Debug)]
pub struct AggregatedStatisticByService {
    pub data: String,
    pub min: i64,
    pub max: i64,
    pub avg: i64,
    pub success_amount: i64,
    pub errors_amount: i64,
    pub amount: i64,
}

pub struct StatisticsRepo {
    postgres: MyPostgres,
}

impl StatisticsRepo {
    pub async fn new(postgres_settings: Arc<dyn PostgresSettings + Sync + Send + 'static>) -> Self {
        Self {
            postgres: MyPostgres::from_settings(APP_NAME, postgres_settings.clone())
                .set_sql_request_timeout(Duration::from_secs(20))
                .with_table_schema_verification::<StatisticsDto>(TABLE_NAME, Some(PK_NAME.into()))
                .build()
                .await,
        }
    }

    pub async fn update_metrics(&self, dto_s: &[StatisticsDto]) {
        self.postgres
            .bulk_insert_or_update_db_entity(
                TABLE_NAME,
                UpdateConflictType::OnPrimaryKeyConstraint(PK_NAME.into()),
                dto_s,
            )
            .await
            .unwrap();
    }

    pub async fn get_aggregated_statistics(&self) -> Vec<AggregatedStatistics> {
        let mut dt = DateTimeAsMicroseconds::now();
        dt.add_days(-2);

        let where_model = StatisticByDateWhereModel {
            date: dt.unix_microseconds,
        };

        let records: Vec<SelectServicesStatisticDto> = self
            .postgres
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap();

        let mut result = Vec::with_capacity(records.len());

        for record in records {
            result.push(AggregatedStatistics {
                service: record.service,
                avg: record.sum_of_duration.get_value() / record.amount.get_value(),
                amount: record.amount.get_value(),
            })
        }

        result
    }

    pub async fn get_aggregated_statistics_of_service(
        &self,
        app: &str,
    ) -> Vec<AggregatedStatisticByService> {
        let mut dt = DateTimeAsMicroseconds::now();
        dt.add_days(-2);

        let where_model = StatisticByDateAndServiceWhereModel {
            date: dt.unix_microseconds,
            data: app,
        };

        let records: Vec<SelectByServiceStatisticDto> = self
            .postgres
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap();

        let mut result = Vec::with_capacity(records.len());

        for record in records {
            result.push(AggregatedStatisticByService {
                data: record.data,
                min: record.min.get_value(),
                max: record.max.get_value(),
                avg: record.sum_of_duration.get_value() / record.amount.get_value(),
                success_amount: record.success_amount.get_value(),
                errors_amount: record.errors_amount.get_value(),
                amount: record.amount.get_value(),
            });
        }

        result
    }

    pub async fn restore(&self, service: &str, app: &str, rounded_hour: i64) -> StatisticsDto {
        let where_model = WhereByHourModel {
            service: service.to_string(),
            data: app.to_string(),
            date: rounded_hour,
        };

        let result = self
            .postgres
            .query_single_row(TABLE_NAME, Some(&where_model))
            .await
            .unwrap();

        match result {
            Some(result) => result,
            None => StatisticsDto {
                service: service.to_string(),
                data: app.to_string(),
                date: rounded_hour,
                max: 0,
                min: 0,
                errors_amount: 0,
                success_amount: 0,
                sum_of_duration: 0,
                amount: 0,
            },
        }
    }
}
