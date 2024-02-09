use crate::{app_ctx::AppContext, postgres::dto::MetricDto};

pub async fn upload_events(app: &AppContext, events: Vec<MetricDto>) {
    app.to_write_queue.enqueue(events).await;
}
