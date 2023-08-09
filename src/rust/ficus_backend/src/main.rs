use ficus_backend::{ficus_proto::backend_service_server::BackendServiceServer, grpc::backend_service::FicusService};
use tonic::transport::Server;

mod event_log;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ficus_service = FicusService {};
    let service = BackendServiceServer::new(ficus_service);
    Server::builder()
        .add_service(service)
        .serve("[::1]:8080".parse()?)
        .await?;

    Ok(())
}
