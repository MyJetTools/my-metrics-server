use my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::db::*;
use crate::ignore_events::IgnoreEvents;

#[derive(MyHttpInput)]
pub struct NewMetricsEvent {
    #[http_body(description = "Metrics")]
    pub body: Vec<NewMetric>,
}

impl NewMetricsEvent {
    pub fn into_dto(self, ignore_events: &IgnoreEvents) -> Vec<MetricDto> {
        let mut result = Vec::with_capacity(self.body.len());
        for mut metric in self.body {
            if ignore_events.event_should_be_ignored(&metric.service_name, &metric.event_data) {
                continue;
            }

            let mut duration = metric.ended - metric.started;
            if duration < 0 {
                duration = 0;
            }

            if let Some(ip) = metric.ip.take() {
                if metric.tags.is_none() {
                    metric.tags = Some(Vec::new());
                }

                metric.tags.as_mut().unwrap().push(MetricHttpTag {
                    key: "ip".to_string(),
                    value: ip,
                });
            }

            let metric_tags = crate::mappers::metric_tags::get(metric.tags.take());

            result.push(MetricDto {
                id: metric.process_id,
                started: metric.started,
                duration_micro: duration,
                name: metric.service_name,
                data: metric.event_data,
                success: metric.success,
                fail: metric.fail,
                tags: metric_tags.tags,
                client_id: metric_tags.client_id,
            })
        }

        result
    }
}

#[derive(Serialize, Deserialize, MyHttpObjectStructure)]
pub struct NewMetric {
    #[serde(rename = "processId")]
    pub process_id: i64,
    #[serde(rename = "started")]
    pub started: i64,

    #[serde(rename = "ended")]
    pub ended: i64,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "eventData")]
    pub event_data: String,
    pub success: Option<String>,
    pub fail: Option<String>,
    pub ip: Option<String>,
    pub tags: Option<Vec<MetricHttpTag>>,
}
#[derive(Serialize, Deserialize, MyHttpObjectStructure)]
pub struct MetricHttpTag {
    pub key: String,
    pub value: String,
}
