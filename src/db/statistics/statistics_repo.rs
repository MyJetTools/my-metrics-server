use my_sqlite::{SqlLiteConnection, SqlLiteConnectionBuilder};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::dto::*;

const TABLE_NAME: &'static str = "statistics";
//const PK_NAME: &'static str = "statistics_pk";

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
    connection: SqlLiteConnection,
}

impl StatisticsRepo {
    pub async fn new(file_name: String) -> Self {
        Self {
            connection: SqlLiteConnectionBuilder::new(file_name)
                .create_table_if_no_exists::<StatisticsDto>(TABLE_NAME)
                .build()
                .await
                .unwrap(),
        }
    }

    pub async fn update_metrics(&self, dto_s: &[StatisticsDto]) {
        self.connection
            .bulk_insert_or_update(dto_s, TABLE_NAME)
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
            .connection
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
            service: app,
        };

        let records: Vec<SelectByServiceStatisticDto> = self
            .connection
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

    pub async fn restore(&self, service: &str, data: &str, rounded_hour: i64) -> StatisticsDto {
        let where_model = WhereByHourModel {
            service: service.to_string(),
            data_hashed: super::super::data_hashed::calc(data),
            date: rounded_hour,
        };

        let result = self
            .connection
            .query_single_row(TABLE_NAME, Some(&where_model))
            .await
            .unwrap();

        match result {
            Some(result) => result,
            None => StatisticsDto {
                service: service.to_string(),
                data_hashed: super::super::data_hashed::calc(data),
                data: data.to_string(),
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
