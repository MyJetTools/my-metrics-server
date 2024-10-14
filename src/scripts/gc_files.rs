use rust_extensions::date_time::{HourKey, IntervalKey};

use crate::app_ctx::AppContext;

pub async fn gc_files(app: &AppContext, from_hour_key: IntervalKey<HourKey>) {
    let files = super::get_metrics_files(app).await;

    //println!("Files: {:?}", files);

    for file in files {
        if let Some(file_hour_key) = file.get_hour_key() {
            if file_hour_key <= from_hour_key {
                app.repo.gc(file_hour_key).await;

                tokio::spawn(async move {
                    let result = tokio::fs::remove_file(file.get_path_and_file_name()).await;

                    if let Err(err) = result {
                        println!(
                            "Error deleting file{}. Err: {:?}",
                            file.get_path_and_file_name(),
                            err
                        );
                    } else {
                        println!("File {} is deleted", file.get_path_and_file_name());
                    }
                });
            }
        }
    }
}
