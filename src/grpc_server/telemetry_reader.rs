use crate::reader_grpc::telemetry_reader_server::TelemetryReader;
use crate::reader_grpc::*;

use super::server::GrpcService;
use my_grpc_extensions::server::generate_server_stream;

#[tonic::async_trait]
impl TelemetryReader for GrpcService {
    generate_server_stream!(stream_name:"GetAvailableHoursAgoStream", item_name:"AvailableFileGrpcModel");

    async fn get_available_hours_ago(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetAvailableHoursAgoStream>, tonic::Status> {
        // let request = request.into_inner();
        let response = crate::flows::get_available_hours_ago(&self.app).await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(response.into_iter(), |dto| dto).await
    }

    generate_server_stream!(stream_name:"GetAppsStream", item_name:"ServiceGrpcModel");

    async fn get_apps(
        &self,
        request: tonic::Request<GetAppsRequest>,
    ) -> Result<tonic::Response<Self::GetAppsStream>, tonic::Status> {
        let request = request.into_inner();
        let overview: Vec<ServiceGrpcModel> =
            crate::flows::get_hour_app_statistics(&self.app, request.hour_key.into()).await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(overview.into_iter(), |dto| dto).await
    }

    generate_server_stream!(stream_name:"GetAppActionsStream", item_name:"AppActionGrpcModel");

    async fn get_app_actions(
        &self,
        request: tonic::Request<GetByAppRequest>,
    ) -> Result<tonic::Response<Self::GetAppActionsStream>, tonic::Status> {
        let request = request.into_inner();

        let result: Vec<AppActionGrpcModel> = crate::flows::get_hour_app_data_statistics(
            &self.app,
            request.hour_key.into(),
            &request.app_id,
        )
        .await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(result.into_iter(), |dto| dto).await
    }

    generate_server_stream!(stream_name:"GetAppEventsByActionStream", item_name:"AppDataGrpcModel");

    async fn get_app_events_by_action(
        &self,
        request: tonic::Request<GetAppEventsByActionRequest>,
    ) -> Result<tonic::Response<Self::GetAppEventsByActionStream>, tonic::Status> {
        let request = request.into_inner();
        let dto_data = self
            .app
            .repo
            .get_by_service_name(request.hour_key.into(), &request.app_id, &request.data)
            .await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(dto_data.into_iter(), |dto| dto.into())
            .await
    }

    generate_server_stream!(stream_name:"GetByProcessIdStream", item_name:"MetricEventGrpcModel");

    async fn get_by_process_id(
        &self,
        request: tonic::Request<GetByProcessIdRequest>,
    ) -> Result<tonic::Response<Self::GetByProcessIdStream>, tonic::Status> {
        let request = request.into_inner();

        let dto_data = self
            .app
            .repo
            .get_by_process_id(request.hour_key.into(), request.process_id)
            .await;

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
