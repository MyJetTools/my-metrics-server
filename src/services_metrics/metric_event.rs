use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct MetricEvent {
    pub started: DateTimeAsMicroseconds,
    pub finished: DateTimeAsMicroseconds,
    pub service_name: String,
    pub service_data: String,
}
