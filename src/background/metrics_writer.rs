use std::sync::Arc;

use rust_extensions::events_loop::EventsLoopTick;

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
    async fn tick(&self, _: ()) {
        while let Some(events_to_write) = self.app.to_write_queue.get_events_to_write(1000).await {
            self.app.repo.insert(&events_to_write).await;
        }
    }
}
