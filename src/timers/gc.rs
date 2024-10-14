use std::sync::Arc;

use rust_extensions::{
    date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey},
    MyTimerTick,
};

use crate::app_ctx::AppContext;

pub struct GcTimer {
    app: Arc<AppContext>,
}

impl GcTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for GcTimer {
    async fn tick(&self) {
        crate::scripts::gc_metrics_pool(&self.app).await;

        let mut now = DateTimeAsMicroseconds::now();
        now.add_hours(-2);

        let hour_key: IntervalKey<HourKey> = now.into();

        crate::scripts::gc_files(&self.app, hour_key).await;

        let mut cache_access = self.app.cache.lock().await;

        cache_access
            .statistics_by_hour_and_service_name
            .gc_old_data(hour_key);

        cache_access.event_amount_by_hours.gc_old_data(hour_key);
    }
}
