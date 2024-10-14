use rust_extensions::date_time::{HourKey, IntervalKey};

use crate::{app_ctx::AppContext, caches::AppDataHourStatistics, db::HourAppDataStatisticsDto};

pub async fn get_hour_app_data_statistics<
    TResult: From<AppDataHourStatistics> + From<HourAppDataStatisticsDto>,
>(
    app_ctx: &AppContext,
    hour_key: IntervalKey<HourKey>,
    app: &str,
) -> Vec<TResult> {
    let result = app_ctx
        .cache
        .lock()
        .await
        .statistics_by_hour_and_service_name
        .get(hour_key, app, |itm| itm.clone().into());

    if let Some(result) = result {
        return result;
    }

    println!(
        "Loading statistics from DB for app: {} with hour_key {}",
        app,
        hour_key.as_i64_ref()
    );

    let from_db = app_ctx
        .hour_app_data_statistics_repo
        .get_by_app(hour_key, app)
        .await;

    from_db.into_iter().map(|itm| itm.into()).collect()
}
