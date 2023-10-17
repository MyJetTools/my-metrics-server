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

impl ServiceOverviewDto {
    pub fn from_metric_dto(src: Vec<MetricDto>) -> Vec<Self> {
        let by_data = rust_extensions::grouped_data::group_to_btree_map(src.into_iter(), |itm| {
            itm.data.clone()
        });

        let mut result = Vec::with_capacity(by_data.len());

        for (data, metrics) in by_data {
            let mut min = None;
            let mut max = None;
            let mut sum = 0;
            let mut success = 0;
            let mut fail = 0;

            let total_metrics_amount = metrics.len();

            for metric in metrics {
                match min {
                    Some(min_value) => {
                        if metric.duration_micro < min_value {
                            min = Some(metric.duration_micro);
                        }
                    }
                    None => {
                        min = Some(metric.duration_micro);
                    }
                }

                match max {
                    Some(max_value) => {
                        if metric.duration_micro > max_value {
                            max = Some(metric.duration_micro);
                        }
                    }
                    None => {
                        max = Some(metric.duration_micro);
                    }
                }

                sum += metric.duration_micro;

                if metric.success.is_some() {
                    success += 1;
                } else {
                    fail += 1;
                }
            }

            let avg = sum / total_metrics_amount as i64;

            result.push(ServiceOverviewDto {
                data,
                min: min.unwrap(),
                max: max.unwrap(),
                avg,
                success,
                fail,
                total: total_metrics_amount,
            });
        }

        result
    }
}

#[derive(SelectDbEntity)]
pub struct CountSelectDto {
    pub count: i32,
}
