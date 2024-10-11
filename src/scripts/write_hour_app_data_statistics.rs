use std::collections::BTreeMap;

use crate::{app_ctx::AppContext, caches::MetricByHour, db::StatisticsDto};

pub async fn write_hour_app_data_statistics(
    app: &AppContext,
    metrics_to_save: BTreeMap<String, BTreeMap<String, BTreeMap<i64, MetricByHour>>>,
) {
    let mut dto_to_insert = Vec::new();

    for (service_name, data) in metrics_to_save.iter() {
        for (action_name, data) in data {
            for (hour, data) in data {
                dto_to_insert.push(StatisticsDto {
                    service: service_name.to_string(),
                    data_hashed: crate::db::data_hashed::calc(action_name),
                    date: *hour,
                    data: action_name.to_string(),
                    max: data.max,
                    min: data.min,
                    errors_amount: data.errors_amount,
                    success_amount: data.success_amount,
                    sum_of_duration: data.sum_of_duration,
                    amount: data.amount,
                })
            }
        }
    }

    app.statistics_repo.update_metrics(&dto_to_insert).await;

    let mut metrics_access = app.cache.lock().await;
    metrics_access
        .aggregated_metrics_cache
        .confirm_metrics_saved(&metrics_to_save);
}
