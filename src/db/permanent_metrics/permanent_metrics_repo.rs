use my_sqlite::{SqlLiteConnection, SqlLiteConnectionBuilder};

use super::PermanentMetricDto;

const TABLE_NAME: &str = "permanent_metrics";
pub struct PermanentMetricsRepo {
    connection: SqlLiteConnection,
}

impl PermanentMetricsRepo {
    pub async fn new(file_name: String) -> Self {
        let connection = SqlLiteConnectionBuilder::new(file_name.to_string())
            .create_table_if_no_exists::<PermanentMetricDto>(TABLE_NAME)
            .build()
            .await
            .unwrap();

        Self { connection }
    }

    pub async fn insert(&self, dto: &[PermanentMetricDto]) {
        self.connection
            .bulk_insert_or_update(&dto, TABLE_NAME)
            .await
            .unwrap();
    }
}
