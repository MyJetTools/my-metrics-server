use crate::{app_ctx::AppContext, postgres::dto::MetricDto};

pub async fn upload_events(app: &AppContext, metrics: Vec<MetricDto>) {
    app.metrics_cache.update(metrics.as_slice()).await;
    app.to_write_queue.enqueue(metrics).await;
}
