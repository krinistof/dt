use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::{Any, CorsLayer}; // For CORS

// Import generated types
// The `include_proto!` macro generates rust code from the proto file
// and includes it in the current scope. The "greeter" part should
// match the package name in your .proto file.
pub mod greeter {
    tonic::include_proto!("greeter");
}

use greeter::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    // CORS layer for gRPC-Web
    // This allows requests from any origin, method, and header.
    // For production, you might want to restrict this.
    let cors = CorsLayer::new()
        .allow_origin(Any) // In production, specify your frontend origin
        .allow_methods(Any)
        .allow_headers(Any);

    // Enable gRPC-Web and apply CORS
    let grpc_service = GreeterServer::new(greeter);
    let grpc_web_service = tonic_web::enable(grpc_service);

    Server::builder()
        .accept_http1(true) // Important for gRPC-Web
        .layer(cors)        // Apply CORS layer
        .add_service(grpc_web_service) // Add the gRPC-Web enabled service
        .serve(addr)
        .await?;

    Ok(())
}
