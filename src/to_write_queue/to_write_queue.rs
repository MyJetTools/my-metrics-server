use std::collections::HashMap;

use rust_extensions::{
    date_time::DateTimeAsMicroseconds,
    sorted_vec::{InsertOrUpdateEntry, SortedVec},
};
use tokio::sync::Mutex;

use crate::db::MetricDto;

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

    pub async fn enqueue(&self, to_push: Vec<MetricDto>) -> HashMap<i64, String> {
        let mut result = HashMap::new();
        let mut write_access = self.metrics.lock().await;
        for new_metric in to_push {
            match write_access.insert_or_update(&new_metric.id) {
                InsertOrUpdateEntry::Insert(insert_entity) => {
                    insert_entity.insert(MetricsChunkByProcessId::new(new_metric));
                }
                InsertOrUpdateEntry::Update(update_entry) => {
                    update_entry.item.push(new_metric);

                    if let Some(client_id) = update_entry.item.client_id.as_ref() {
                        if result.contains_key(&update_entry.item.process_id) {
                            result.insert(update_entry.item.process_id, client_id.to_string());
                        }
                    }
                }
            }
        }

        result
    }

    pub async fn get_events_to_write(&self, max_amount: usize) -> Option<Vec<MetricDto>> {
        let now = DateTimeAsMicroseconds::now();
        let mut write_access = self.metrics.lock().await;

        let mut ready_to_go = Vec::new();
        let mut amount = 0;

        for itm in write_access.iter_mut() {
            if (now - itm.created).get_full_seconds() > 5 {
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
                result.extend(chunk.items);
            }
        }

        Some(result)
    }
}
