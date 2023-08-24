use crate::services_metrics::ServicesMetrics;
use rust_extensions::AppStates;
use std::sync::Arc;
use tokio::sync::Mutex;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub process_id: String,
    pub metrics: Mutex<ServicesMetrics>,
}

impl AppContext {
    pub fn new() -> AppContext {
        AppContext {
            app_states: Arc::new(AppStates::create_initialized()),
            process_id: uuid::Uuid::new_v4().to_string(),
            metrics: Mutex::new(ServicesMetrics::new()),
        }
    }
}
