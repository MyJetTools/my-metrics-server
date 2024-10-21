use my_sqlite::macros::*;

use crate::db::{EventTagDto, MetricDto};

#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct PermanentMetricDto {
    #[db_index(id:0, index_name:"process_id_idx", is_unique:false, order:"ASC")]
    pub id: i64,
    #[primary_key(0)]
    pub client_id: String,
    #[db_index(id:0, index_name:"started_idx", is_unique:false, order:"ASC")]
    #[primary_key(1)]
    pub started: i64,
    pub duration_micro: i64,
    #[primary_key(2)]
    pub name: String,
    #[primary_key(3)]
    pub data: String,
    pub success: Option<String>,
    pub fail: Option<String>,
    pub tags: Option<Vec<EventTagDto>>,
}

impl Into<PermanentMetricDto> for MetricDto {
    fn into(self) -> PermanentMetricDto {
        PermanentMetricDto {
            id: self.id,
            client_id: self.client_id.unwrap_or_default(),
            started: self.started,
            duration_micro: self.duration_micro,
            name: self.name,
            data: self.data,
            success: self.success,
            fail: self.fail,
            tags: self.tags,
        }
    }
}
