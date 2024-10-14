use std::sync::Arc;

use rust_extensions::{events_loop::EventsLoopTick, StopWatch};

use crate::app_ctx::AppContext;

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

            /*
            println!(
                "MetricsWriter written {} metrics in: {:?}",
                events_amount,
                sw.duration()
            );
             */

            let mut cache_write_access = self.app.cache.lock().await;

            for (interval_key, grouped) in &items {
                cache_write_access
                    .statistics_by_hour_and_service_name
                    .update(*interval_key, grouped);

                for metric_dto in grouped {
                    cache_write_access
                        .event_amount_by_hours
                        .inc(*interval_key, metric_dto);
                }
            }
        }
    }
}
