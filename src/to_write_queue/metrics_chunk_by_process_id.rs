use rust_extensions::{date_time::DateTimeAsMicroseconds, sorted_vec::EntityWithKey};

use crate::db::MetricDto;

pub struct MetricsChunkByProcessId {
    pub process_id: i64,
    pub client_id: Option<String>,
    pub items: Vec<MetricDto>,
    pub created: DateTimeAsMicroseconds,
}

impl MetricsChunkByProcessId {
    pub fn new(metric_dto: MetricDto) -> Self {
        let client_id = metric_dto.client_id.clone();
        Self {
            process_id: metric_dto.id,
            client_id,
            items: vec![metric_dto],
            created: DateTimeAsMicroseconds::now(),
        }
    }

    pub fn push(&mut self, new_metric: MetricDto) {
        if self.client_id.is_none() {
            if let Some(client_id) = new_metric.client_id.clone() {
                self.client_id = Some(client_id);
            }
        }

        self.items.push(new_metric);
    }
}

impl EntityWithKey<i64> for MetricsChunkByProcessId {
    fn get_key(&self) -> &i64 {
        &self.process_id
    }
}
