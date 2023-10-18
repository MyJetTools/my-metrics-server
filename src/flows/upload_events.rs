use crate::{app_ctx::AppContext, postgres::dto::MetricDto};

pub async fn upload_events(app: &AppContext, metrics: Vec<MetricDto>) {
    app.to_write_queue.enqueue(metrics).await;
}
