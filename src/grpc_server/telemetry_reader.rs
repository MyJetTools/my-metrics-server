use crate::reader_grpc::telemetry_reader_server::TelemetryReader;
use crate::reader_grpc::*;

use super::server::GrpcService;
use my_grpc_extensions::server::generate_server_stream;
use rust_extensions::date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey};

#[tonic::async_trait]
impl TelemetryReader for GrpcService {
    generate_server_stream!(stream_name:"GetAppsStream", item_name:"ServiceGrpcModel");

    async fn get_apps(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetAppsStream>, tonic::Status> {
        let now = DateTimeAsMicroseconds::now();
        let hour_key: IntervalKey<HourKey> = now.into();

        let overview = self.app.hour_statistics_repo.get(hour_key).await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(overview.into_iter(), |dto| {
            ServiceGrpcModel {
                id: dto.app,
                avg: dto.duration_micros / dto.amount,
                amount: dto.amount,
            }
        })
        .await
    }

    generate_server_stream!(stream_name:"GetAppActionsStream", item_name:"AppActionGrpcModel");

    async fn get_app_actions(
        &self,
        request: tonic::Request<GetByAppRequest>,
    ) -> Result<tonic::Response<Self::GetAppActionsStream>, tonic::Status> {
        let request = request.into_inner();

        let result = self
            .app
            .statistics_repo
            .get_aggregated_statistics_of_service(&request.app_id)
            .await;

        /*
        let mut from = DateTimeAsMicroseconds::now();

        from.add_days(-1);

        let dto_data = self
            .app
            .repo
            .get_service_overview(&request.app_id, from)
            .await;

         */

        my_grpc_extensions::grpc_server::send_vec_to_stream(result.into_iter(), |dto| {
            AppActionGrpcModel {
                data: dto.data,
                min: dto.min,
                avg: dto.avg,
                max: dto.max,
                success: dto.success_amount,
                error: dto.errors_amount,
                total: dto.amount,
            }
        })
        .await
    }

    generate_server_stream!(stream_name:"GetAppEventsByActionStream", item_name:"AppDataGrpcModel");

    async fn get_app_events_by_action(
        &self,
        request: tonic::Request<GetAppEventsByActionRequest>,
    ) -> Result<tonic::Response<Self::GetAppEventsByActionStream>, tonic::Status> {
        let request = request.into_inner();

        let mut from = DateTimeAsMicroseconds::now();

        from.add_days(-1);

        let dto_data = self
            .app
            .repo
            .get_by_service_name(&request.app_id, &request.data)
            .await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(dto_data.into_iter(), |dto| {
            AppDataGrpcModel {
                process_id: dto.id,
                started: dto.started,
                duration: dto.duration_micro,
                success: dto.success,
                fail: dto.fail,
                tags: if let Some(dto_tags) = dto.tags {
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
                },
            }
        })
        .await
    }

    generate_server_stream!(stream_name:"GetByProcessIdStream", item_name:"MetricEventGrpcModel");

    async fn get_by_process_id(
        &self,
        request: tonic::Request<GetByProcessIdRequest>,
    ) -> Result<tonic::Response<Self::GetByProcessIdStream>, tonic::Status> {
        let request = request.into_inner();

        let mut from = DateTimeAsMicroseconds::now();

        from.add_days(-1);

        let dto_data = self.app.repo.get_by_process_id(request.process_id).await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(dto_data.into_iter(), |dto| {
            MetricEventGrpcModel {
                started: dto.started,
                duration: dto.duration_micro,
                success: dto.success,
                name: dto.name,
                data: dto.data,
                fail: dto.fail,
                tags: if let Some(dto_tags) = dto.tags {
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
                },
            }
        })
        .await
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
