use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetServices {
    pub names: Vec<String>,
}
