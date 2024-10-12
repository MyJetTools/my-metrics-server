use std::collections::BTreeMap;

use rust_extensions::sorted_vec::SortedVecWithStrKey;

use crate::{app_ctx::AppContext, db::HourStatisticsDto, events_amount_by_hour::StatisticsByHour};

pub async fn write_hour_statistics_to_db(
    app: &AppContext,
    metrics_to_save: BTreeMap<i64, SortedVecWithStrKey<StatisticsByHour>>,
) {
    let mut dto = Vec::new();

    for (hour_key, items) in metrics_to_save {
        for item in items.into_vec() {
            dto.push(HourStatisticsDto {
                hour_key,
                app: item.name,
                duration_micros: item.duration_micros,
                amount: item.amount,
            });
        }
    }

    app.hour_statistics_repo.update(dto.as_slice()).await;
}
