use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone)]
pub struct MetricEvent {
    pub id: i64,
    pub started: DateTimeAsMicroseconds,
    pub finished: DateTimeAsMicroseconds,
    pub service_name: String,
    pub event_data: String,
    pub success: Option<String>,
    pub fail: Option<String>,
    pub ip: Option<String>,
}

impl MetricEvent {
    pub fn get_duration_mcs(&self) -> i64 {
        self.finished.unix_microseconds - self.started.unix_microseconds
    }

    pub fn is_success(&self) -> bool {
        self.success.is_some()
    }

    pub fn is_fail(&self) -> bool {
        self.fail.is_some()
    }
}
