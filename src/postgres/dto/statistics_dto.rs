use my_postgres::{macros::*, GroupByMax, GroupByMin, GroupBySum};

#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct StatisticsDto {
    #[primary_key(0)]
    #[generate_where_model("WhereByHourModel")]
    pub service: String,
    #[primary_key(1)]
    #[generate_where_model("WhereByHourModel")]
    pub data_hashed: String,
    #[primary_key(2)]
    #[generate_where_model("WhereByHourModel")]
    #[order_by_desc]
    pub date: i64,

    pub data: String,
    pub max: i64,
    pub min: i64,
    pub errors_amount: i64,
    pub success_amount: i64,
    pub sum_of_duration: i64,
    pub amount: i64,
}

#[derive(SelectDbEntity)]
pub struct SelectServicesStatisticDto {
    #[group_by]
    pub service: String,
    pub sum_of_duration: GroupBySum<i64>,
    pub amount: GroupBySum<i64>,
}

#[derive(SelectDbEntity)]
pub struct SelectByServiceStatisticDto {
    #[group_by]
    pub data: String,
    pub min: GroupByMin<i64>,
    pub max: GroupByMax<i64>,
    pub sum_of_duration: GroupBySum<i64>,
    pub success_amount: GroupBySum<i64>,
    pub errors_amount: GroupBySum<i64>,
    pub amount: GroupBySum<i64>,
}

#[derive(WhereDbModel)]
pub struct StatisticByDateWhereModel {
    #[operator(">=")]
    pub date: i64,
}

#[derive(WhereDbModel)]
pub struct StatisticByDateAndServiceWhereModel<'s> {
    pub service: &'s str,
    #[operator(">=")]
    pub date: i64,
}
