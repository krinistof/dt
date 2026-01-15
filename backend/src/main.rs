use std::net::SocketAddr;

use anyhow::Result;
use axum::{Router, routing::any_service};
use connectrpc_axum::ConnectRequest;
use dt::db;
use dt::{
    init_logging,
    //dt_connectrpc, dt_proto::dtservice,
    log_connectrpc,
    log_proto::logcollectorservice,
};
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_logging();

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    let log_service = logcollectorservice::LogCollectorServiceBuilder::new()
        .log(log_connectrpc)
        .build();

    //let log_grpc_web_service = tonic_web::enable(log_grpc_service);

    /*
    let sqlite = db::new().await?;
    let dt_instance = DtInstance::new(sqlite);
    let dt_grpc_service = DtServiceServer::new(dt_instance);
    let dt_grpc_web_service = tonic_web::enable(dt_grpc_service);
    */

    let static_files_service = any_service(ServeDir::new("../frontend/dist"));
    let media_files_service = any_service(ServeDir::new("media"));

    let router = Router::new()
        .fallback_service(log_service)
        //.fallback_service(dt_service)
        //.nest_service("/", dt_service)
        .nest_service("/media", media_files_service)
        .fallback(static_files_service);

    axum::serve(listener, router).await?;

    Ok(())
}
