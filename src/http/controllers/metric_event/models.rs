use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

#[derive(MyHttpInput)]
pub struct NewMetricsEvent {
    #[http_body_raw(description = "Metrics")]
    pub body: my_http_server::types::RawDataTyped<Vec<NewMetric>>,
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
