use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::{Any, CorsLayer}; // For CORS

// Import generated types
// The `include_proto!` macro generates rust code from the proto file
// and includes it in the current scope. The "log" part should
// match the package name in your .proto file.
pub mod log {
    tonic::include_proto!("log");
}

use log::{
    log_service_server::{LogService, LogServiceServer},
    LogReply, LogRequest,
};

#[derive(Debug, Default)]
pub struct MyLogService {}

#[tonic::async_trait]
impl LogService for MyLogService {
    async fn log(
        &self,
        request: Request<LogRequest>,
    ) -> Result<Response<LogReply>, Status> {
        for message in request.get_ref().messages.iter() {
            println!("Log: {}", message);
        }

        let reply = LogReply {
            success: true,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let log_service = MyLogService::default();

    println!("LogService listening on {}", addr);

    // CORS layer for gRPC-Web
    // This allows requests from any origin, method, and header.
    // TODO DEV ONLY
    let cors = CorsLayer::new()
        .allow_origin(Any) // In production, specify your frontend origin
        .allow_methods(Any)
        .allow_headers(Any);

    // Enable gRPC-Web and apply CORS
    let grpc_service = LogServiceServer::new(log_service);
    let grpc_web_service = tonic_web::enable(grpc_service);

    Server::builder()
        .accept_http1(true) // Important for gRPC-Web
        .layer(cors)        // Apply CORS layer
        .add_service(grpc_web_service) // Add the gRPC-Web enabled service
        .serve(addr)
        .await?;

    Ok(())
}