use crate::{
    postgres::dto::{EventTagDto, MetricDto},
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
            ip: None,
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
