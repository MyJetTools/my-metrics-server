use my_sqlite::{SqlLiteConnection, SqlLiteConnectionBuilder};
use rust_extensions::date_time::{HourKey, IntervalKey};

use super::dto::*;

const TABLE_NAME: &'static str = "statistics";
//const PK_NAME: &'static str = "statistics_pk";

pub struct HourAppDataStatisticsRepo {
    connection: SqlLiteConnection,
}

impl HourAppDataStatisticsRepo {
    pub async fn new(file_name: String) -> Self {
        Self {
            connection: SqlLiteConnectionBuilder::new(file_name)
                .create_table_if_no_exists::<HourAppDataStatisticsDto>(TABLE_NAME)
                .build()
                .await
                .unwrap(),
        }
    }

    pub async fn update_metrics(&self, dto_s: &[HourAppDataStatisticsDto]) {
        println!("Updating metrics for {:?} records", dto_s);
        self.connection
            .bulk_insert_or_update(dto_s, TABLE_NAME)
            .await
            .unwrap();
    }

    pub async fn get(&self, hour_key: IntervalKey<HourKey>) -> Vec<HourAppDataStatisticsDto> {
        let where_model = WhereByHourModel {
            hour_key: hour_key.to_i64(),
        };

        self.connection
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap()
    }

    pub async fn get_by_app(
        &self,
        hour_key: IntervalKey<HourKey>,
        app: &str,
    ) -> Vec<HourAppDataStatisticsDto> {
        let where_model = GetByHourAndServiceWhereModel {
            hour_key: hour_key.to_i64(),
            service: app,
        };

        self.connection
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap()
    }
}
