use my_sqlite::macros::*;

#[derive(TableSchema, InsertDbEntity, UpdateDbEntity, SelectDbEntity, Debug)]
pub struct HourStatisticsDto {
    #[primary_key(0)]
    #[generate_where_model(name = "WhereByHourKey")]
    pub hour_key: i64,
    #[primary_key(1)]
    pub app: String,
    pub duration_micros: i64,
    pub amount: i64,
}
