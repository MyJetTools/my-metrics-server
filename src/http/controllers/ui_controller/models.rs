use my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::db::*;

#[derive(Debug, MyHttpInput)]
pub struct GetServicesHttpInput {
    #[http_query(description = "Hour key")]
    pub hour_key: i64,
}

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct GetServicesResponse {
    pub services: Vec<ServiceHttpModel>,
}

#[derive(Deserialize, Serialize, MyHttpObjectStructure)]
pub struct ServiceHttpModel {
    pub id: String,
    pub avg: i64,
    pub amount: i64,
}
/////////

#[derive(Debug, MyHttpInput)]
pub struct GetServiceMetricsOverview {
    #[http_query(description = "Id of service")]
    pub id: String,

    #[http_query(description = "Hour key")]
    pub hour_key: i64,
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
    pub success: i64,
    pub error: i64,
    pub total: i64,
}

////////////

#[derive(Debug, MyHttpInput)]
pub struct GetByServiceDataRequest {
    #[http_query(description = "Id of service")]
    pub id: String,
    #[http_query(description = "Data of the service")]
    pub data: String,
    #[http_query(name:"hourKey", description = "Hour Key")]
    pub hour_key: i64,
    #[http_query(name:"clientId", description = "Client Id")]
    pub client_id: Option<String>,

    #[http_query(name:"fromSecondWithinHour", description = "Second within hour")]
    pub from_second_within_hour: i64,
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

impl Into<MetricHttpModel> for MetricDto {
    fn into(self) -> MetricHttpModel {
        MetricHttpModel {
            id: self.id,
            started: self.started,
            duration: self.duration_micro,
            success: self.success,
            error: self.fail,
            ip: if let Some(tags) = self.tags {
                format!("{:?}", tags).into()
            } else {
                None
            },
        }
    }
}

#[derive(Debug, MyHttpInput)]
pub struct GetByProcessIdRequest {
    #[http_query(name: "processId"; description = "Id of service")]
    pub process_id: i64,
    #[http_query(name: "hour_key"; description = "Hour key")]
    pub hour_key: i64,
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

impl Into<MetricByProcessModel> for MetricDto {
    fn into(self) -> MetricByProcessModel {
        MetricByProcessModel {
            id: self.name,
            data: self.data,
            started: self.started,
            duration: self.duration_micro,
            success: self.success,
            error: self.fail,
            ip: if let Some(tags) = self.tags {
                format!("{:?}", tags).into()
            } else {
                None
            },
        }
    }
}
