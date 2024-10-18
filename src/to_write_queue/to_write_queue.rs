use ahash::AHashMap;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

use crate::{app_ctx::StatisticsCache, db::MetricDto};

use super::MetricsChunkByProcessId;

pub struct ToWriteQueue {
    pub metrics: Mutex<AHashMap<i64, MetricsChunkByProcessId>>,
}

impl ToWriteQueue {
    pub fn new() -> Self {
        Self {
            metrics: Mutex::new(AHashMap::new()),
        }
    }

    pub async fn enqueue(
        &self,
        to_push: Vec<MetricDto>,
        mut lazy_lock: crate::lazy_lock::LazyLock<'_, StatisticsCache>,
    ) {
        let mut write_access = self.metrics.lock().await;
        for new_metric in to_push {
            if let Some(client_id) = new_metric.client_id.as_ref() {
                lazy_lock
                    .get_mut()
                    .await
                    .process_id_user_id_links
                    .update(new_metric.id, client_id);
            }

            if let Some(entity) = write_access.get_mut(&new_metric.id) {
                entity.push(new_metric);
            } else {
                write_access.insert(new_metric.id, MetricsChunkByProcessId::new(new_metric));
            }
        }
    }

    pub async fn get_events_to_write(
        &self,
        max_amount: usize,
    ) -> Option<Vec<MetricsChunkByProcessId>> {
        let now = DateTimeAsMicroseconds::now();
        let mut write_access = self.metrics.lock().await;

        let mut ready_to_go = Vec::new();
        let mut amount = 0;

        for itm in write_access.values_mut() {
            if (now - itm.created).get_full_seconds() >= 10 {
                ready_to_go.push(itm.process_id);
                amount += itm.items.len();

                if amount > max_amount {
                    break;
                }
            }
        }

        let mut result = Vec::with_capacity(amount);

        for process_id in ready_to_go {
            if let Some(chunk) = write_access.remove(&process_id) {
                result.push(chunk);
            }
        }

        Some(result)
    }

    pub async fn get_queue_and_capacity_and_by_process_capacity(&self) -> SizeAndCapacity {
        let read_access = self.metrics.lock().await;

        let mut len = 0;
        let mut capacity = 0;
        for itm in read_access.values() {
            len += itm.items.len();
            capacity += itm.items.capacity();
        }
        SizeAndCapacity {
            events_queue_size: len,
            events_capacity_size: capacity,
            process_queue_size: read_access.len(),
            process_queue_capacity: read_access.capacity(),
        }
    }
}

pub struct SizeAndCapacity {
    pub events_queue_size: usize,
    pub events_capacity_size: usize,
    pub process_queue_size: usize,
    pub process_queue_capacity: usize,
}
