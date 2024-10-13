use std::collections::BTreeMap;

use rust_extensions::date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey};

use crate::{
    app_ctx::{AppContext, METRICS_FILE_PREFIX},
    reader_grpc::*,
};

pub async fn get_available_hours_ago(app: &AppContext) -> Vec<AvailableFileGrpcModel> {
    let path_to_scan = app.settings_reader.get_db_path().await;

    let mut dir_entry = tokio::fs::read_dir(path_to_scan).await.unwrap();

    let now = DateTimeAsMicroseconds::now();

    let mut result = BTreeMap::new();

    while let Some(entry) = dir_entry.next_entry().await.unwrap() {
        if !entry.path().is_file() {
            continue;
        }

        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                println!("file_name: {}", file_name);
                if entry.path().starts_with(METRICS_FILE_PREFIX) {
                    continue;
                }

                let hour_key = get_hour_key(file_name);

                if let Some(hour_key) = hour_key {
                    if let Ok(hour) = hour_key.try_to_date_time() {
                        let diff = now - hour;

                        let hours = diff.get_full_hours();

                        let file_metadata = entry.metadata().await.unwrap();
                        result.insert(hours, file_metadata.len());
                    }
                }
            }
        }
    }

    result
        .into_iter()
        .map(|(hour_ago, file_size)| AvailableFileGrpcModel {
            hour_ago,
            file_size,
        })
        .collect()
}

fn get_hour_key(path_str: &str) -> Option<IntervalKey<HourKey>> {
    if path_str.len() < 18 {
        return None;
    }

    let file_index = &path_str[8..18];

    let result = file_index.parse::<i64>().ok()?;
    Some(result.into())
}
