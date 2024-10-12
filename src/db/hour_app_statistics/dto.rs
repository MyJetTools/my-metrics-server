use my_sqlite::macros::*;

#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct HourAppDataStatisticsDto {
    #[primary_key(0)]
    #[generate_where_model("WhereByHourModel")]
    #[order_by_desc]
    pub hour_key: i64,
    #[primary_key(1)]
    pub service: String,
    #[primary_key(2)]
    pub data_hashed: String,

    pub data: String,
    pub max: i64,
    pub min: i64,
    pub errors_amount: i64,
    pub success_amount: i64,
    pub sum_of_duration: i64,
    pub amount: i64,
}

#[derive(WhereDbModel, Debug)]
pub struct GetByHourAndServiceWhereModel<'s> {
    pub hour_key: i64,
    pub service: &'s str,
}
