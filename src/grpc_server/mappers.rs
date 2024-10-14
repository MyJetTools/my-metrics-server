use crate::{
    caches::AppDataHourStatistics, db::*, events_amount_by_hour::StatisticsByHour, reader_grpc::*,
    writer_grpc::*,
};

impl Into<MetricDto> for TelemetryGrpcEvent {
    fn into(self) -> MetricDto {
        MetricDto {
            id: self.process_id,
            started: self.started_at,
            duration_micro: self.finished_at - self.started_at,
            name: self.service_name,
            data: self.event_data,
            success: self.success,
            fail: self.fail,
            tags: if self.tags.len() > 0 {
                let result: Vec<_> = self
                    .tags
                    .into_iter()
                    .map(|src| EventTagDto {
                        key: src.key,
                        value: src.value,
                    })
                    .collect();

                result.into()
            } else {
                None
            },
        }
    }
}

impl From<AppDataHourStatistics> for AppActionGrpcModel {
    fn from(value: AppDataHourStatistics) -> Self {
        let total = value.success_amount + value.errors_amount;
        Self {
            data: value.data,
            min: value.min,
            avg: value.sum_of_duration / total,
            max: value.max,
            success: value.success_amount,
            error: value.errors_amount,
            total,
        }
    }
}

impl From<HourAppDataStatisticsDto> for AppActionGrpcModel {
    fn from(value: HourAppDataStatisticsDto) -> Self {
        let total = value.success_amount + value.errors_amount;
        Self {
            data: value.data,
            min: value.min,
            avg: value.sum_of_duration / total,
            max: value.max,
            success: value.success_amount,
            error: value.errors_amount,
            total,
        }
    }
}

impl From<StatisticsByHour> for ServiceGrpcModel {
    fn from(value: StatisticsByHour) -> Self {
        Self {
            id: value.name,
            avg: value.duration_micros / value.amount,
            amount: value.amount,
        }
    }
}

impl From<HourStatisticsDto> for ServiceGrpcModel {
    fn from(value: HourStatisticsDto) -> Self {
        Self {
            id: value.app,
            avg: value.duration_micros / value.amount,
            amount: value.amount,
        }
    }
}
