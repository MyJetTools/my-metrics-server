use std::{collections::BTreeMap, sync::Arc};

use my_sqlite::{SqlLiteConnection, SqlLiteConnectionBuilder};
use rust_extensions::date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey};
use tokio::sync::Mutex;

use super::dto::MetricDto;

pub struct SqlLitePoolItem {
    pub last_access: DateTimeAsMicroseconds,
    pub connection: Arc<Mutex<SqlLiteConnection>>,
    pub file_name: String,
}

impl SqlLitePoolItem {
    pub async fn new(file_name: String) -> Self {
        let connection = SqlLiteConnectionBuilder::new(file_name.to_string())
            .create_table_if_no_exists::<MetricDto>(super::TABLE_NAME)
            .build()
            .await
            .unwrap();

        Self {
            last_access: DateTimeAsMicroseconds::now(),
            connection: Arc::new(Mutex::new(connection)),
            file_name,
        }
    }
}

pub struct SqlLitePool {
    file_name_prefix: String,
    pool: Mutex<BTreeMap<IntervalKey<HourKey>, SqlLitePoolItem>>,
}

impl SqlLitePool {
    pub fn new(file_name_prefix: String) -> Self {
        Self {
            file_name_prefix,
            pool: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn get_for_read_access(
        &self,
        hour_key: IntervalKey<HourKey>,
    ) -> Option<Arc<Mutex<SqlLiteConnection>>> {
        let mut write_access = self.pool.lock().await;

        if let Some(pool_item) = write_access.get_mut(&hour_key) {
            pool_item.last_access = DateTimeAsMicroseconds::now();

            return Some(pool_item.connection.clone());
        }

        let file_name = compile_file_name(&self.file_name_prefix, hour_key);

        let file_info = tokio::fs::metadata(&file_name).await;

        if file_info.is_err() {
            return None;
        }

        let item = SqlLitePoolItem::new(file_name).await;

        let result = item.connection.clone();
        write_access.insert(hour_key, item);

        Some(result)
    }

    pub async fn get_for_write_access(
        &self,
        hour_key: IntervalKey<HourKey>,
    ) -> Arc<Mutex<SqlLiteConnection>> {
        let mut write_access = self.pool.lock().await;

        if let Some(pool_item) = write_access.get_mut(&hour_key) {
            pool_item.last_access = DateTimeAsMicroseconds::now();
            return pool_item.connection.clone();
        }

        let file_name = compile_file_name(&self.file_name_prefix, hour_key);

        let item = SqlLitePoolItem::new(file_name).await;

        let result = item.connection.clone();
        write_access.insert(hour_key, item);

        result
    }

    pub async fn get_last(&self) -> Option<Arc<Mutex<SqlLiteConnection>>> {
        let mut write_access = self.pool.lock().await;
        if let Some((_, itm)) = write_access.iter_mut().last() {
            itm.last_access = DateTimeAsMicroseconds::now();
            return Some(itm.connection.clone());
        }
        None
    }

    pub async fn get_all(&self) -> Vec<Arc<Mutex<SqlLiteConnection>>> {
        let mut result = Vec::new();
        let write_access = self.pool.lock().await;
        for (_, item) in write_access.iter() {
            result.insert(0, item.connection.clone());
        }
        result
    }

    pub async fn gc(&self, from_dt: DateTimeAsMicroseconds) {
        let from_hour_key: IntervalKey<HourKey> = from_dt.into();

        let mut write_access = self.pool.lock().await;

        let mut to_gc = Vec::new();
        for key in write_access.keys() {
            if *key < from_hour_key {
                to_gc.push(*key);
            } else {
                break;
            }
        }

        for key in to_gc {
            let file_name = {
                let item = write_access.remove(&key);

                item.map(|i| i.file_name)
            };

            if let Some(file_name) = file_name {
                tokio::fs::remove_file(file_name).await.unwrap();
            }
        }
    }
}

fn compile_file_name(file_name_prefix: &str, hour_key: IntervalKey<HourKey>) -> String {
    format!("{}-{}.db", file_name_prefix, hour_key.to_i64())
}
