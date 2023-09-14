use std::time::Duration;

use crate::postgres::dto::*;
use crate::writer_grpc::telemetry_writer_server::TelemetryWriter;
use crate::writer_grpc::*;

const READ_TIMEOUT: Duration = Duration::from_secs(10);
use super::server::GrpcService;

#[tonic::async_trait]
impl TelemetryWriter for GrpcService {
    async fn upload(
        &self,
        request: tonic::Request<tonic::Streaming<TelemetryGrpcEvent>>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let events = my_grpc_extensions::read_grpc_stream::as_vec_with_transformation(
            request,
            READ_TIMEOUT,
            &|grpc_model| {
                let result: MetricDto = grpc_model.into();
                result
            },
        )
        .await
        .unwrap();

        if let Some(events) = events {
            self.app.to_write_queue.enqueue(events).await;
        }

        Ok(tonic::Response::new(()))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
