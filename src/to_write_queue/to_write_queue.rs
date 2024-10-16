use rust_extensions::{
    date_time::DateTimeAsMicroseconds,
    sorted_vec::{InsertOrUpdateEntry, SortedVec},
};
use tokio::sync::Mutex;

use crate::{app_ctx::StatisticsCache, db::MetricDto};

use super::MetricsChunkByProcessId;

pub struct ToWriteQueue {
    pub metrics: Mutex<SortedVec<i64, MetricsChunkByProcessId>>,
}

impl ToWriteQueue {
    pub fn new() -> Self {
        Self {
            metrics: Mutex::new(SortedVec::new()),
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

            match write_access.insert_or_update(&new_metric.id) {
                InsertOrUpdateEntry::Insert(insert_entity) => {
                    insert_entity.insert(MetricsChunkByProcessId::new(new_metric));
                }
                InsertOrUpdateEntry::Update(update_entry) => {
                    update_entry.item.push(new_metric);
                }
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

        for itm in write_access.iter_mut() {
            if (now - itm.created).get_full_seconds() >= 3 {
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

    pub async fn get_queue_and_capacity_and_by_process_capacity(&self) -> (usize, usize, usize) {
        let read_access = self.metrics.lock().await;

        let mut len = 0;
        let mut capacity = 0;
        for itm in read_access.iter() {
            len += itm.items.len();
            capacity += itm.items.capacity();
        }
        (len, capacity, read_access.capacity())
    }
}
