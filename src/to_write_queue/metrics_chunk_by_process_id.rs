use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::EntityWithKey};

use crate::db::MetricDto;

pub struct MetricsChunkByProcessId {
    pub process_id: i64,
    pub items: Vec<MetricDto>,
    pub created: DateTimeAsMicroseconds,
}

impl MetricsChunkByProcessId {
    pub fn new(metric_dto: MetricDto) -> Self {
        Self {
            process_id: metric_dto.id,

            items: vec![metric_dto],
            created: DateTimeAsMicroseconds::now(),
        }
    }

    pub fn push(&mut self, new_metric: MetricDto) {
        self.items.push(new_metric);
    }
}

impl EntityWithKey<i64> for MetricsChunkByProcessId {
    fn get_key(&self) -> &i64 {
        &self.process_id
    }
}
