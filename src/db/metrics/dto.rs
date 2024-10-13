use my_sqlite::macros::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde_derive::{Deserialize, Serialize};

#[derive(TableSchema, InsertDbEntity, SelectDbEntity, Debug)]
pub struct MetricDto {
    #[generate_where_model("WhereByProcessId")]
    #[generate_where_model(name:"GcWhereModel", operator = "<")]
    #[order_by_desc]
    #[db_index(id:0, index_name:"process_id_idx", is_unique:false, order:"ASC")]
    pub id: i64,

    #[db_index(id:0, index_name:"started_idx", is_unique:false, order:"ASC")]
    #[generate_where_model(name:"FromStartedWhereModel", operator = ">")]
    #[generate_where_model(name:"FromStartedAndServiceNameWhereModel", operator = ">")]
    #[primary_key(2)]
    pub started: i64,
    pub duration_micro: i64,
    #[primary_key(0)]
    #[generate_where_model(name:"FromStartedAndServiceNameWhereModel", as_str)]
    pub name: String,
    #[primary_key(1)]
    pub data: String,
    pub success: Option<String>,
    pub fail: Option<String>,
    pub tags: Option<Vec<EventTagDto>>,
}

impl MetricDto {
    pub fn get_started(&self) -> DateTimeAsMicroseconds {
        DateTimeAsMicroseconds::new(self.started)
    }

    pub fn get_tag_value(&self, key: &str) -> Option<&str> {
        let tags = self.tags.as_ref()?;

        for itm in tags {
            if itm.key == key {
                return Some(&itm.value);
            }
        }

        None
    }

    pub fn add_tag(&mut self, key: String, value: String) {
        if let Some(tags) = self.tags.as_mut() {
            tags.push(EventTagDto { key, value });
        } else {
            self.tags = Some(vec![EventTagDto { key, value }]);
        }
    }
}

#[derive(Serialize, Deserialize, DbJsonModel, Debug)]
pub struct EventTagDto {
    pub key: String,
    pub value: String,
}

#[derive(WhereDbModel)]
pub struct WhereByServiceName<'s> {
    pub name: &'s str,
    pub data: &'s str,
    #[limit]
    pub limit: usize,
}
