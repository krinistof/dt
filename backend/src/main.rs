use axum::{routing::any_service, Router};
use tonic::{Request, Response, Status};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

pub mod log {
    tonic::include_proto!("log.v1");
}

use log::{
    log_collector_service_server::{LogCollectorService, LogCollectorServiceServer},
    LogRequest, LogResponse,
};

#[derive(Debug, Default)]
pub struct LogCollector {}

#[tonic::async_trait]
impl LogCollectorService for LogCollector {
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        println!("Log: {}", request.get_ref().message);

        let reply = LogResponse { success: true };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:80".parse()?;
    let log_service = LogCollector::default();

    println!("WebService listening on {addr}");

    let grpc_service = LogCollectorServiceServer::new(log_service);
    let grpc_web_service = tonic_web::enable(grpc_service);

    let static_files_service = any_service(ServeDir::new("../frontend/dist"));
    let media_files_service = any_service(ServeDir::new("media"));

    let app = Router::new()
        .nest_service("/grpc", grpc_web_service)
        .nest_service("/media", media_files_service)
        .fallback(static_files_service)
        .layer(
            //TODO DEV ONLY
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
