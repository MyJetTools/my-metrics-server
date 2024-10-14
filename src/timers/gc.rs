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
        let duration = self.app.settings_reader.get_hours_to_gc().await;

        let gc_from = DateTimeAsMicroseconds::now().sub(duration);

        let hour_key: IntervalKey<HourKey> = gc_from.into();

        //println!("GC hour is: {}", hour_key.to_i64());

        crate::scripts::gc_files(&self.app, hour_key).await;

        let mut cache_access = self.app.cache.lock().await;

        cache_access
            .statistics_by_hour_and_service_name
            .gc_old_data(hour_key);

        cache_access.event_amount_by_hours.gc_old_data(hour_key);
    }
}
