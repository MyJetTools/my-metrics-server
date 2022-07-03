use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetServicesResponse {
    pub services: Vec<ServiceModel>,
}

#[derive(Deserialize, Serialize)]
pub struct ServiceModel {
    pub id: String,
    pub avg: i64,
}
