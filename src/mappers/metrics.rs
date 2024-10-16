use crate::{
    caches::{AppDataHourStatistics, AppDurationStatistics},
    db::*,
    reader_grpc::*,
    writer_grpc::*,
};

impl Into<MetricDto> for TelemetryGrpcEvent {
    fn into(self) -> MetricDto {
        let metric_tags = super::metric_tags::get(Some(self.tags));
        MetricDto {
            id: self.process_id,
            started: self.started_at,
            duration_micro: self.finished_at - self.started_at,
            name: self.service_name,
            data: self.event_data,
            success: self.success,
            fail: self.fail,
            tags: metric_tags.tags,
            client_id: metric_tags.client_id,
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

impl From<AppDurationStatistics> for ServiceGrpcModel {
    fn from(value: AppDurationStatistics) -> Self {
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

impl Into<AppDataGrpcModel> for MetricDto {
    fn into(mut self) -> AppDataGrpcModel {
        let tags = super::metric_tags::to_tag_grpc_model(&mut self);

        AppDataGrpcModel {
            process_id: self.id,
            started: self.started,
            duration: self.duration_micro,
            success: self.success,
            fail: self.fail,
            tags,
        }
    }
}

impl Into<MetricEventGrpcModel> for MetricDto {
    fn into(mut self) -> MetricEventGrpcModel {
        let tags = super::metric_tags::to_tag_grpc_model(&mut self);
        MetricEventGrpcModel {
            started: self.started,
            duration: self.duration_micro,
            success: self.success,
            name: self.name,
            data: self.data,
            fail: self.fail,
            tags,
        }
    }
}
