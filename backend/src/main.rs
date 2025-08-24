use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::{Any, CorsLayer};

pub mod log {
    tonic::include_proto!("log.v1");
}

use log::{
    log_collector_service_server::{LogCollectorService, LogCollectorServiceServer},
    LogResponse, LogRequest,
};

#[derive(Debug, Default)]
pub struct LogCollector {}

#[tonic::async_trait]
impl LogCollectorService for LogCollector {
    async fn log(
        &self,
        request: Request<LogRequest>,
    ) -> Result<Response<LogResponse>, Status> {
        println!("Log: {}", request.get_ref().message);

        let reply = LogResponse {
            success: true,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let log_service = LogCollector::default();

    println!("LogService listening on {addr}");

    // CORS layer for gRPC-Web
    // This allows requests from any origin, method, and header.
    // TODO DEV ONLY
    let cors = CorsLayer::new()
        .allow_origin(Any) // In production, specify your frontend origin
        .allow_methods(Any)
        .allow_headers(Any);

    let grpc_service = LogCollectorServiceServer::new(log_service);
    let grpc_web_service = tonic_web::enable(grpc_service);

    Server::builder()
        .accept_http1(true) // Important for gRPC-Web
        .layer(cors)        // Apply CORS layer
        .add_service(grpc_web_service) // Add the gRPC-Web enabled service
        .serve(addr)
        .await?;

    Ok(())
}
