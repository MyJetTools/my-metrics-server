use std::collections::{BTreeMap, HashMap};

use crate::{app_ctx::AppContext, db::HourStatisticsDto, events_amount_by_hour::StatisticsByHour};

pub async fn write_hour_statistics_to_db(
    app: &AppContext,
    metrics_to_save: BTreeMap<i64, HashMap<String, StatisticsByHour>>,
) {
    let mut dto = Vec::new();

    for (hour_key, items) in metrics_to_save {
        for (app, statistics) in items {
            dto.push(HourStatisticsDto {
                hour_key,
                app,
                duration_micros: statistics.duration_micros,
                amount: statistics.amount,
            });
        }
    }

    app.hour_statistics_repo.update(dto.as_slice()).await;
}
