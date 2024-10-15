use crate::{
    caches::AppDataHourStatistics, db::*, events_amount_by_hour::StatisticsByHour, reader_grpc::*,
    writer_grpc::*,
};

use super::metric_tags::CLIENT_ID_TAG;

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

impl Into<AppDataGrpcModel> for MetricDto {
    fn into(self) -> AppDataGrpcModel {
        let mut tags = if let Some(dto_tags) = self.tags {
            let mut result = Vec::with_capacity(dto_tags.len());

            for dto_tag in dto_tags {
                result.push(TagGrpcModel {
                    key: dto_tag.key,
                    value: dto_tag.value,
                });
            }

            result
        } else {
            vec![]
        };

        if let Some(client_id) = self.client_id {
            tags.push(TagGrpcModel {
                key: CLIENT_ID_TAG.to_string(),
                value: client_id,
            });
        }

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
