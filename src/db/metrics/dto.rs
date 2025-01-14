use my_sqlite::macros::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde_derive::{Deserialize, Serialize};

#[derive(TableSchema, InsertDbEntity, SelectDbEntity, Debug)]
pub struct MetricDto {
    #[generate_where_model("WhereByProcessId")]
    #[generate_where_model(name:"GcWhereModel", operator = "<")]
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
    pub client_id: Option<String>,
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

    pub fn remove_tag_value(&mut self, key: &str) -> Option<EventTagDto> {
        let tags = self.tags.as_mut()?;

        let index = tags.iter().position(|x| x.key == key)?;

        let result = tags.remove(index);

        Some(result)
    }

    pub fn update_user_id_to_client_id(&mut self, user_id_tag: &str, client_id_tag: &str) {
        if let Some(tags) = &mut self.tags {
            let index = tags.iter().position(|x| x.key == user_id_tag);

            if let Some(index) = index {
                let user_id = tags.remove(index);
                tags.push(EventTagDto {
                    key: client_id_tag.to_string(),
                    value: user_id.value,
                });
            }
        }
    }

    pub fn add_tag(&mut self, key: String, value: String) -> &str {
        if let Some(tags) = self.tags.as_mut() {
            tags.push(EventTagDto { key, value });
        } else {
            self.tags = Some(vec![EventTagDto { key, value }]);
        }

        self.tags.as_ref().unwrap().last().unwrap().value.as_str()
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
    #[ignore_if_none]
    pub client_id: Option<&'s str>,
    #[operator(">=")]
    #[ignore_if_none]
    pub started: Option<i64>,

    #[limit]
    pub limit: usize,
}
