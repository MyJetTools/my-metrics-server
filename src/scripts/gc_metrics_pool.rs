use std::sync::Arc;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;

pub async fn gc_metrics_pool(app: &Arc<AppContext>) {
    let duration_before_now = app.settings_reader.get_duration_before_now_to_gc().await;

    let as_seconds = duration_before_now.as_secs() as i64;

    let mut gc_before = DateTimeAsMicroseconds::now();

    gc_before.add_seconds(-as_seconds);

    println!("Executing GC from: {}", gc_before.to_rfc3339());

    app.repo.gc(gc_before).await;

    println!("Executed GC");
}
