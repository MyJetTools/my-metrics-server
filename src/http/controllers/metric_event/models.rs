use my_http_server::HttpFailResult;
use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::postgres::dto::MetricDto;

#[derive(MyHttpInput)]
pub struct NewMetricsEvent {
    #[http_body_raw(description = "Metrics")]
    pub body: my_http_server::types::RawDataTyped<Vec<NewMetric>>,
}

impl NewMetricsEvent {
    pub fn into_dto(self) -> Result<Vec<MetricDto>, HttpFailResult> {
        let metrics = self.body.deserialize_json()?;

        let mut result: Vec<MetricDto> = Vec::with_capacity(metrics.len());

        for metric in metrics {
            let mut duration = metric.ended - metric.started;
            if duration < 0 {
                duration = 0;
            }
            result.push(MetricDto {
                id: metric.process_id,
                started: metric.started,
                duration_micro: duration,
                name: metric.service_name,
                data: metric.event_data,
                success: metric.success,
                fail: metric.fail,
                ip: metric.ip,
                tags: None,
            })
        }

        Ok(result)
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
}
