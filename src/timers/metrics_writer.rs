use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::{
    app_ctx::{AppContext, StatisticsCache},
    db::MetricDto,
    to_write_queue::MetricsChunkByProcessId,
};

pub struct MetricsWriter {
    app: Arc<AppContext>,
}

impl MetricsWriter {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
#[async_trait::async_trait]
impl MyTimerTick for MetricsWriter {
    async fn tick(&self) {
        let started = DateTimeAsMicroseconds::now();
        while let Some(chunks) = self.app.to_write_queue.get_events_to_write(1000).await {
            let mut events_to_write = Vec::with_capacity(1000);

            {
                let mut cache_access = self.app.cache.lock().await;
                for chunk in chunks {
                    populate_client_id(chunk, &mut cache_access, &mut events_to_write).await;
                }
            }

            //let events_amount = events_to_write.len();

            let items = self.app.repo.insert(events_to_write).await;

            /*
            println!(
                "MetricsWriter written {} metrics in: {:?}",
                events_amount,
                sw.duration()
            );
             */

            let mut cache_write_access = self.app.cache.lock().await;

            for (interval_key, grouped) in &items {
                cache_write_access
                    .statistics_by_app_and_data
                    .update(*interval_key, grouped);

                for metric_dto in grouped {
                    cache_write_access
                        .event_amount_by_hours
                        .inc(*interval_key, metric_dto);
                }
            }

            if (DateTimeAsMicroseconds::now() - started).get_full_seconds() >= 20 {
                break;
            }

            cache_write_access.process_id_user_id_links.gc();
        }
    }
}

async fn populate_client_id<'s>(
    chunk: MetricsChunkByProcessId,
    cache: &'s mut StatisticsCache,
    out_put: &mut Vec<MetricDto>,
) {
    let len = chunk.items.len();
    if let Some(client_id) = cache
        .process_id_user_id_links
        .resolve_user_id(chunk.process_id)
    {
        for mut metric in chunk.items {
            if metric.name == "dashboard-rest-api" {
                println!(
                    "{}. Chunk: {}. Has client_id for process_id: {:?}",
                    DateTimeAsMicroseconds::now().unix_microseconds,
                    len,
                    metric
                );
            }
            if metric.client_id.is_none() {
                metric.client_id = Some(client_id.to_string());
            }

            out_put.push(metric);
        }
        return;
    }

    for metric in chunk.items {
        if metric.name == "dashboard-rest-api" {
            println!(
                "{}. Chunk:{}, No client_id for process_id: {:?}",
                DateTimeAsMicroseconds::now().unix_microseconds,
                len,
                metric
            );
        }
        out_put.push(metric);
    }
}
