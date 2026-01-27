use std::net::SocketAddr;
use anyhow::Result;

use dt::{
    init_logging,
    UqServer, static_files_service,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_logging();

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    // Initialize UqServer with local SQLite
    let db_url = std::env::var("DATABASE_URL").unwrap_or("sqlite://db/dt.db?mode=rwc".into());
    let uq_server = UqServer::new(&db_url).await?;
    
    let app = static_files_service().merge(uq_server.into_router());

    axum::serve(listener, app).await?;

    Ok(())
}
