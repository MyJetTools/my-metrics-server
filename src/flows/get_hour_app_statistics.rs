use rust_extensions::date_time::{HourKey, IntervalKey};

use crate::{app_ctx::AppContext, caches::*, db::HourStatisticsDto};

pub async fn get_hour_app_statistics<
    TResult: From<AppDurationStatistics> + From<HourStatisticsDto>,
>(
    app: &AppContext,
    hour_key: IntervalKey<HourKey>,
) -> Vec<TResult> {
    if let Some(result) = app
        .cache
        .lock()
        .await
        .event_amount_by_hours
        .get(hour_key, |itm| itm.clone().into())
    {
        return result;
    }

    let result = app.hour_statistics_repo.get(hour_key).await;

    result.into_iter().map(|itm| itm.into()).collect()
}
