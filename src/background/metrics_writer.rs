use std::sync::Arc;

use rust_extensions::events_loop::EventsLoopTick;

use crate::{app_ctx::AppContext, caches::MetricByHour};

pub struct MetricsWriter {
    app: Arc<AppContext>,
}

impl MetricsWriter {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
#[async_trait::async_trait]
impl EventsLoopTick<()> for MetricsWriter {
    async fn started(&self) {}

    async fn finished(&self) {}

    async fn tick(&self, _: ()) {
        while let Some(events_to_write) = self.app.to_write_queue.get_events_to_write(1000).await {
            self.app.repo.insert(&events_to_write).await;

            let mut write_access = self.app.metrics_cache.lock().await;

            for metric_dto in &events_to_write {
                if let Some(to_update) = write_access.get_to_update(metric_dto) {
                    to_update.update(metric_dto);
                    continue;
                }

                let rounded_hour = metric_dto.get_rounded_hour();

                let restored = self
                    .app
                    .statistics_repo
                    .restore(&metric_dto.name, &metric_dto.data, rounded_hour)
                    .await;

                let mut restored: MetricByHour = restored.into();

                restored.update(metric_dto);

                write_access.restore(&metric_dto.name, &metric_dto.data, rounded_hour, restored);
            }
        }
    }
}
