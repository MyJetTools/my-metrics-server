use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::app_ctx::AppContext;

pub struct GcMetricsTimer {
    app: Arc<AppContext>,
}

impl GcMetricsTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]

impl MyTimerTick for GcMetricsTimer {
    async fn tick(&self) {
        let count = self.app.repo.get_events_amount().await;

        let min_events_to_keep = self
            .app
            .settings_reader
            .get_min_events_to_keep_before_gc()
            .await;

        if count < min_events_to_keep {
            return;
        }

        let duration_before_now = self
            .app
            .settings_reader
            .get_duration_before_now_to_gc()
            .await;

        let as_seconds = duration_before_now.as_secs() as i64;

        let mut gc_before = DateTimeAsMicroseconds::now();

        gc_before.add_seconds(-as_seconds);

        self.app.repo.gc(gc_before).await;
    }
}
