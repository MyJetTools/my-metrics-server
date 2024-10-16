use crate::reader_grpc::telemetry_reader_server::TelemetryReader;
use crate::reader_grpc::*;

use super::server::GrpcService;
use my_grpc_extensions::server::generate_server_stream;
use rust_extensions::date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey};

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

        let hour_key: IntervalKey<HourKey> = request.hour_key.into();
        let from_started = if request.from_sec_within_hour == 0 {
            None
        } else {
            let mut dt: DateTimeAsMicroseconds = hour_key.try_to_date_time().unwrap();
            dt.add_seconds(request.from_sec_within_hour);
            Some(dt.unix_microseconds)
        };

        let client_id = if request.client_id.is_empty() {
            None
        } else {
            Some(request.client_id.as_str())
        };

        let dto_data = self
            .app
            .repo
            .get_by_service_name(
                hour_key,
                &request.app_id,
                &request.data,
                client_id,
                from_started,
            )
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

        my_grpc_extensions::grpc_server::send_vec_to_stream(dto_data.into_iter(), |dto| dto.into())
            .await
    }

    async fn get_tech_metrics(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<TechMetricsGrpcModel>, tonic::Status> {
        let response = {
            let cache_read_access = self.app.cache.lock().await;

            let queue_size = cache_read_access
                .statistics_by_hour_and_service_name
                .get_size_and_capacity();

            let queue_hours_size = cache_read_access
                .statistics_by_hour_and_service_name
                .get_queue_hours_size();

            let user_id_links_size = cache_read_access.process_id_user_id_links.get_size();

            TechMetricsGrpcModel {
                queue_hours_size: queue_hours_size.0 as u64,
                queue_to_persist_hours_size: queue_hours_size.1 as u64,
                queue_size: queue_size.0 as u64,
                queue_capacity: queue_size.1 as u64,
                user_id_links_size: user_id_links_size as u64,
            }
        };

        Ok(tonic::Response::new(response))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
