use crate::{app_ctx::AppContext, db::*};

pub async fn upload_events(app: &AppContext, events: Vec<MetricDto>) {
    let lazy_lock = crate::lazy_lock::LazyLock::new(&app.cache);

    app.to_write_queue.enqueue(events, lazy_lock).await;

    /*
    if maps.len() > 0 {
        let mut cache_access = app.cache.lock().await;

        for (process_id, client_id) in maps {
            cache_access
                .process_id_user_id_links
                .update(process_id, client_id.as_str());
        }
    }
     */
}

/*
async fn populate_client_id(app: &AppContext, events: &mut Vec<MetricDto>) {
    let has_with_no_client_id = events.iter().any(|event| event.client_id.is_none());

    if !has_with_no_client_id {
        return;
    }

    let cache_access = app.cache.lock().await;

    for event in events.iter_mut() {
        if event.client_id.is_none() {
            if let Some(client_id) = cache_access
                .process_id_user_id_links
                .resolve_user_id(event.id)
            {
                event.client_id = Some(client_id.to_string());
            }
        }
    }
}
 */
