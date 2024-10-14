use crate::{
    app_ctx::{AppContext, METRICS_FILE_PREFIX},
    metric_file::MetricFile,
};

pub async fn get_metrics_files(app: &AppContext) -> Vec<MetricFile> {
    let path_to_scan = app.settings_reader.get_db_path().await;

    let mut dir_entry = tokio::fs::read_dir(path_to_scan).await.unwrap();

    let mut result = Vec::new();

    while let Some(entry) = dir_entry.next_entry().await.unwrap() {
        if !entry.path().is_file() {
            continue;
        }

        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                if file_name.starts_with(METRICS_FILE_PREFIX) {
                    let file_metadata = entry.metadata().await.unwrap();
                    result.push(MetricFile::new(file_name.to_string(), file_metadata.len()));
                }

                /*
                let hour_key = get_hour_key(file_name);

                if let Some(hour_key) = hour_key {
                    if let Ok(hour) = hour_key.try_to_date_time() {
                        let diff = now - hour;

                        let hours = diff.get_full_hours();

                        let file_metadata = entry.metadata().await.unwrap();
                        result.insert(hours, file_metadata.len());
                    }
                }
                 */
            }
        }
    }

    result
}
