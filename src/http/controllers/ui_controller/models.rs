use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct GetServicesResponse {
    pub services: Vec<ServiceModel>,
}

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct ServiceModel {
    pub id: String,
    pub avg: i64,
}
/////////

#[derive(Debug, MyHttpInput)]
pub struct GetServiceMetricsOverview {
    #[http_query(description = "Id of service")]
    pub id: String,
}

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct GetServiceOverviewResponse {
    pub data: Vec<ServiceOverviewContract>,
}
#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct ServiceOverviewContract {
    pub data: String,
    pub min: i64,
    pub max: i64,
    pub avg: i64,
    pub success: usize,
    pub error: usize,
    pub total: usize,
}

////////////

#[derive(Debug, MyHttpInput)]
pub struct GetByServiceDataRequest {
    #[http_query(description = "Id of service")]
    pub id: String,
    #[http_query(description = "Id of service")]
    pub data: String,
}
#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct MetricsResponse {
    pub metrics: Vec<MetricHttpModel>,
}

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct MetricHttpModel {
    pub id: i64,
    pub started: i64,
    pub duration: i64,

    pub success: Option<String>,
    pub error: Option<String>,
    pub ip: Option<String>,
}

////////
#[derive(Debug, MyHttpInput)]
pub struct GetByProcessIdRequest {
    #[http_query(name: "processId"; description = "Id of service")]
    pub process_id: i64,
}

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct MetricsByProcessResponse {
    pub metrics: Vec<MetricByProcessModel>,
}

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct MetricByProcessModel {
    pub id: String,
    pub data: String,
    pub started: i64,
    pub duration: i64,
    pub success: Option<String>,
    pub error: Option<String>,
    pub ip: Option<String>,
}
