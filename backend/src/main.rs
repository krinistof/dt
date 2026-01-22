use std::net::SocketAddr;
use anyhow::Result;

use dt::{
    init_logging,
    connect_router, static_files_service,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_logging();

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    /*
    let sqlite = db::new().await?;
    let dt_instance = DtInstance::new(sqlite);
    let dt_grpc_service = DtServiceServer::new(dt_instance);
    let dt_grpc_web_service = tonic_web::enable(dt_grpc_service);
    */

    let sqlite = dt::db::new().await?;
    let app = static_files_service().merge(connect_router(sqlite));

    axum::serve(listener, app).await?;

    Ok(())
}
