use my_sqlite::*;
use rust_extensions::date_time::{HourKey, IntervalKey};

use super::*;

const TABLE_NAME: &str = "hour_statistics";
pub struct HourStatisticsRepo {
    pub sqlite: SqlLiteConnection,
}
impl HourStatisticsRepo {
    pub async fn new(file_name: String) -> Self {
        println!("Creating HourStatisticsRepo with file_name: {}", file_name);
        Self {
            sqlite: SqlLiteConnectionBuilder::new(file_name)
                .create_table_if_no_exists::<HourStatisticsDto>(TABLE_NAME)
                .build()
                .await
                .unwrap(),
        }
    }
    pub async fn update(&self, dto_s: &[HourStatisticsDto]) {
        self.sqlite
            .bulk_insert_or_update(dto_s, TABLE_NAME)
            .await
            .unwrap();
    }

    pub async fn get(&self, hour: IntervalKey<HourKey>) -> Vec<HourStatisticsDto> {
        let where_model = WhereByHourKey {
            hour_key: hour.to_i64(),
        };

        self.sqlite
            .query_rows(TABLE_NAME, Some(&where_model))
            .await
            .unwrap()
    }
}
