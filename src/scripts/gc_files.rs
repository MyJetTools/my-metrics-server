use rust_extensions::date_time::{HourKey, IntervalKey};

use crate::app_ctx::AppContext;

pub async fn gc_files(app: &AppContext, from_hour_key: IntervalKey<HourKey>) {
    let files = super::get_metrics_files(app).await;

    println!("Files: {:?}", files);

    for file in files {
        if let Some(hour_key) = file.get_hour_key() {
            if hour_key <= from_hour_key {
                println!("Would delete file file: {:?}", file);
            }
        }
    }
}
