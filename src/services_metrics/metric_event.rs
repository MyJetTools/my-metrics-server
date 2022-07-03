use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct MetricEvent {
    pub started: DateTimeAsMicroseconds,
    pub finished: DateTimeAsMicroseconds,
    pub service_name: String,
    pub event_data: String,
    pub success: Option<String>,
    pub fail: Option<String>,
}

impl MetricEvent {
    pub fn duration_mcs(&self) -> i64 {
        self.finished.unix_microseconds - self.started.unix_microseconds
    }
}
