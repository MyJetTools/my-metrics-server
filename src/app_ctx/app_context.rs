use crate::services_metrics::ServiesMetrics;
use my_logger::MyLogger;
use rust_extensions::AppStates;
use std::sync::Arc;
use tokio::sync::Mutex;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub logger: Arc<MyLogger>,
    pub process_id: String,
    pub metrics: Mutex<ServiesMetrics>,
}

impl AppContext {
    pub fn new() -> AppContext {
        AppContext {
            app_states: Arc::new(AppStates::create_initialized()),
            logger: Arc::new(MyLogger::to_console()),
            process_id: uuid::Uuid::new_v4().to_string(),
            metrics: Mutex::new(ServiesMetrics::new()),
        }
    }
}
