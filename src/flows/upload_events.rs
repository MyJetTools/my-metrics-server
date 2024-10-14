use crate::{app_ctx::AppContext, db::*};

const USER_ID_TAG: &str = "user_id";
const CLIENT_ID_TAG: &str = "client_id";

pub async fn upload_events(app: &AppContext, mut events: Vec<MetricDto>) {
    populate_user_id(app, &mut events).await;
    app.to_write_queue.enqueue(events).await;
}

async fn populate_user_id(app: &AppContext, events: &mut Vec<MetricDto>) {
    let mut cache_write_access = app.cache.lock().await;

    for event in events {
        let mut has_user_id = false;

        if let Some(user_id) = event.get_tag_value(USER_ID_TAG) {
            cache_write_access
                .process_id_user_id_links
                .update(event.id, user_id);

            has_user_id = true;
        }

        if has_user_id {
            event.update_user_id_to_client_id(USER_ID_TAG, CLIENT_ID_TAG);
            continue;
        }

        if let Some(client_id) = event.get_tag_value(CLIENT_ID_TAG) {
            cache_write_access
                .process_id_user_id_links
                .update(event.id, client_id);
        } else {
            if let Some(user_id) = cache_write_access
                .process_id_user_id_links
                .resolve_user_id(event.id)
            {
                event.add_tag(CLIENT_ID_TAG.to_string(), user_id.to_string());
            }
        }
    }
}
