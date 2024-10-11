use std::sync::Arc;

use rust_extensions::{events_loop::EventsLoopTick, StopWatch};

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
            //let events_amount = events_to_write.len();
            let mut sw = StopWatch::new();
            sw.start();
            let items = self.app.repo.insert(events_to_write).await;
            sw.pause();
            //  println!(
            //      "MetricsWriter written {} metrics in: {:?}",
            //      events_amount,
            //       sw.duration()
            //  );

            let mut cache_write_access = self.app.cache.lock().await;

            for (interval_key, grouped) in &items {
                for metric_dto in grouped {
                    cache_write_access
                        .event_amount_by_hours
                        .inc(*interval_key, metric_dto);

                    if let Some(to_update) = cache_write_access
                        .aggregated_metrics_cache
                        .get_to_update(metric_dto)
                    {
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

                    cache_write_access.aggregated_metrics_cache.restore(
                        &metric_dto.name,
                        &metric_dto.data,
                        rounded_hour,
                        restored,
                    );
                }
            }
        }
    }
}
