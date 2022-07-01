use std::sync::Arc;

use rust_extensions::MyTimerTick;

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
        let mut write_access = self.app.metrics.lock().await;

        write_access.gc();
    }
}
