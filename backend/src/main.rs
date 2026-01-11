use anyhow::Result;
use axum::{Router, routing::any_service};
use dt::db;
use dt::{
    DtInstance, LogCollector, init_logging,
    dt_proto::dt_service_server::DtServiceServer,
    log_proto::log_collector_service_server::LogCollectorServiceServer,
};
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_logging();

    let addr = "0.0.0.0:8080".parse()?;
    info!("WebService listening on {}", addr);

    let log_collector_service = LogCollector::default();
    let log_grpc_service = LogCollectorServiceServer::new(log_collector_service);
    let log_grpc_web_service = tonic_web::enable(log_grpc_service);

    let sqlite = db::new().await?;
    let dt_instance = DtInstance::new(sqlite);
    let dt_grpc_service = DtServiceServer::new(dt_instance);
    let dt_grpc_web_service = tonic_web::enable(dt_grpc_service);

    let static_files_service = any_service(ServeDir::new("../frontend/dist"));
    let media_files_service = any_service(ServeDir::new("media"));

    let app = Router::new()
        .nest_service("/grpc/log", log_grpc_web_service)
        .nest_service("/grpc/dt", dt_grpc_web_service)
        .nest_service("/media", media_files_service)
        .fallback(static_files_service);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
