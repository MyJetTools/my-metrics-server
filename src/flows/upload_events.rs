use crate::{app_ctx::AppContext, db::*};

pub async fn upload_events(app: &AppContext, events: Vec<MetricDto>) {
    println!("Uploading events: {}", events.len());
    app.to_write_queue.enqueue(events).await;
}
