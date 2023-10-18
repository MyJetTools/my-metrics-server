use std::sync::Arc;

use rust_extensions::MyTimerTick;

use crate::{app_ctx::AppContext, postgres::dto::StatisticsDto};

pub struct SaveStatisticsTimer {
    app: Arc<AppContext>,
}

impl SaveStatisticsTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for SaveStatisticsTimer {
    async fn tick(&self) {
        let metrics_to_save = {
            let metrics_access = self.app.metrics_cache.lock().await;
            metrics_access.get_metrics_to_save()
        };

        if metrics_to_save.is_none() {
            return;
        }

        let metrics_to_save = metrics_to_save.unwrap();

        let mut dto_to_insert = Vec::new();

        for (service_name, data) in metrics_to_save.iter() {
            for (action_name, data) in data {
                for (hour, data) in data {
                    dto_to_insert.push(StatisticsDto {
                        service: service_name.to_string(),
                        data: action_name.to_string(),
                        date: *hour,
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

        self.app
            .statistics_repo
            .update_metrics(&dto_to_insert)
            .await;

        let mut metrics_access = self.app.metrics_cache.lock().await;
        metrics_access.confirm_metrics_saved(&metrics_to_save);
    }
}
