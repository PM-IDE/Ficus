use std::sync::Arc;

use ficus_backend::{
    ficus_proto::grpc_backend_service_server::GrpcBackendServiceServer, grpc::backend_service::FicusService,
    pipelines::types::Types,
};
use tonic::transport::Server;

mod event_log;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ficus_service = FicusService::new(Arc::new(Box::new(Types::new())));
    let service = GrpcBackendServiceServer::new(ficus_service);
    Server::builder()
        .add_service(service)
        .serve("[::1]:8080".parse()?)
        .await?;

    Ok(())
}
