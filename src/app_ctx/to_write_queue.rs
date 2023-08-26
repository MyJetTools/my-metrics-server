use std::collections::VecDeque;

use rust_extensions::events_loop::EventsLoop;
use tokio::sync::Mutex;

use crate::postgres::dto::MetricDto;

pub struct ToWriteQueue {
    pub metrics: Mutex<VecDeque<MetricDto>>,

    pub events_loop: EventsLoop<()>,
}

impl ToWriteQueue {
    pub fn new() -> Self {
        Self {
            metrics: Mutex::new(VecDeque::new()),
            events_loop: EventsLoop::new("ToQueueWriter".to_string()),
        }
    }

    pub async fn enqueue(&self, to_push: Vec<MetricDto>) {
        {
            let mut write_access = self.metrics.lock().await;
            for metric in to_push {
                write_access.push_back(metric);
            }
        }

        self.events_loop.send(());
    }

    pub async fn get_events_to_write(&self, max_amount: usize) -> Option<Vec<MetricDto>> {
        let mut write_access = self.metrics.lock().await;

        if write_access.len() == 0 {
            return None;
        }

        let mut result = Vec::with_capacity(max_amount);

        while result.len() < max_amount {
            let metric = write_access.pop_front();

            if metric.is_none() {
                break;
            }

            result.push(metric.unwrap());
        }

        Some(result)
    }
}
