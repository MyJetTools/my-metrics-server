use my_http_server::HttpFailResult;
use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::postgres::dto::{EventTagDto, MetricDto};

#[derive(MyHttpInput)]
pub struct NewMetricsEvent {
    #[http_body_raw(description = "Metrics")]
    pub body: my_http_server::types::RawDataTyped<Vec<NewMetric>>,
}

impl NewMetricsEvent {
    pub fn into_dto(self) -> Result<Vec<MetricDto>, HttpFailResult> {
        let metrics = self.body.deserialize_json()?;

        let mut result: Vec<MetricDto> = Vec::with_capacity(metrics.len());

        for mut metric in metrics {
            let mut duration = metric.ended - metric.started;
            if duration < 0 {
                duration = 0;
            }

            let mut tags = None;

            if let Some(http_tags) = metric.tags.take() {
                for http_tag in http_tags {
                    if tags.is_none() {
                        tags = Some(Vec::new());
                    }

                    tags.as_mut().unwrap().push(EventTagDto {
                        key: http_tag.key,
                        value: http_tag.value,
                    });
                }
            }

            if let Some(ip) = metric.ip {
                if tags.is_none() {
                    tags = Some(Vec::new());
                }

                tags.as_mut().unwrap().push(EventTagDto {
                    key: "ip".to_string(),
                    value: ip,
                });
            }

            result.push(MetricDto {
                id: metric.process_id,
                started: metric.started,
                duration_micro: duration,
                name: metric.service_name,
                data: metric.event_data,
                success: metric.success,
                fail: metric.fail,
                tags,
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
    pub tags: Option<Vec<MetricHttpTags>>,
}
#[derive(Serialize, Deserialize, MyHttpObjectStructure)]
pub struct MetricHttpTags {
    pub key: String,
    pub value: String,
}
