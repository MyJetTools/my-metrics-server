use my_postgres::macros::*;
use my_postgres::GroupByAvg;
use serde_derive::{Deserialize, Serialize};

#[derive(TableSchema, InsertDbEntity, SelectDbEntity, Debug)]
pub struct MetricDto {
    #[primary_key(0)]
    #[generate_where_model("WhereByProcessId")]
    #[generate_where_model(name:"GcWhereModel", operator = "<")]
    #[order_by_desc]
    pub id: i64,
    #[primary_key(1)]
    #[db_index(id:0, index_name:"started_idx", is_unique:false, order:"ASC")]
    #[generate_where_model(name:"FromStartedWhereModel", operator = ">")]
    #[generate_where_model(name:"FromStartedAndServiceNameWhereModel", operator = ">")]
    pub started: i64,
    pub duration_micro: i64,
    #[primary_key(2)]
    #[db_index(id:0, index_name:"name_idx", is_unique:false, order:"ASC")]
    #[generate_where_model(name:"FromStartedAndServiceNameWhereModel", as_str)]
    pub name: String,
    pub data: String,
    pub success: Option<String>,
    pub fail: Option<String>,
    pub tags: Option<Vec<EventTagDto>>,
}

impl MetricDto {
    pub fn get_rounded_hour(&self) -> i64 {
        round_by_hour(self.started)
    }
}

#[derive(Serialize, Deserialize, MyPostgresJsonModel, Debug)]
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

#[derive(SelectDbEntity)]
pub struct ServiceDto {
    #[group_by]
    pub name: String,

    #[db_column_name("duration_micro")]
    pub avg: GroupByAvg<i64>,
}

#[derive(Debug)]
pub struct ServiceOverviewDto {
    pub data: String,
    pub min: i64,
    pub max: i64,
    pub avg: i64,
    pub success: usize,
    pub fail: usize,
    pub total: usize,
}

#[derive(SelectDbEntity)]
pub struct CountSelectDto {
    pub count: i32,
}

pub fn round_by_hour(micro_seconds: i64) -> i64 {
    micro_seconds - micro_seconds % 3600_000_000
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::round_by_hour;

    #[test]
    fn test_round_by_hour() {
        let dt = DateTimeAsMicroseconds::from_str("2015-01-05:12:43.23.123").unwrap();

        let rounded = round_by_hour(dt.unix_microseconds);

        let dest = DateTimeAsMicroseconds::new(rounded);

        assert_eq!(&dest.to_rfc3339()[..19], "2015-01-05T12:00:00");
    }
}
