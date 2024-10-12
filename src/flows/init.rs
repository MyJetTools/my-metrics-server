use rust_extensions::date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey};

use crate::app_ctx::AppContext;

pub async fn init(app: &AppContext) {
    let now = DateTimeAsMicroseconds::now();
    let hour_key: IntervalKey<HourKey> = now.into();

    let hour_app_items = app.hour_statistics_repo.get(hour_key).await;

    let hour_app_data_items = app.hour_app_data_statistics_repo.get(hour_key).await;

    let mut cache_write_access = app.cache.lock().await;
    cache_write_access
        .statistics_by_hour_and_service_name
        .restore(hour_key, hour_app_data_items);

    cache_write_access
        .event_amount_by_hours
        .restore(hour_key, hour_app_items.into_iter().map(|itm| itm.into()));
}
