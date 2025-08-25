use anyhow::Result;
use axum::{routing::any_service, Router};
use tonic::{Request, Response, Status};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;

pub mod log {
    tonic::include_proto!("log.v1");
}

pub mod dt {
    tonic::include_proto!("dt.v1");
}

use log::{
    log_collector_service_server::{LogCollectorService, LogCollectorServiceServer},
    LogRequest, LogResponse,
};

use dt::{
    dt_server::{Dt, DtServer},
    SubmitEventRequest, SubmitEventResponse,
};

#[derive(Debug, Default)]
pub struct LogCollector {}

#[tonic::async_trait]
impl LogCollectorService for LogCollector {
    #[tracing::instrument]
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        info!("Log: {}", request.get_ref().message);

        let reply = LogResponse { success: true };

        Ok(Response::new(reply))
    }
}

#[derive(Debug)]
pub struct DtService {
    db: db::Db,
}

impl DtService {
    pub fn new(db: db::Db) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl Dt for DtService {
    #[tracing::instrument(skip(self))]
    async fn submit_event(
        &self,
        request: Request<SubmitEventRequest>,
    ) -> Result<Response<SubmitEventResponse>, Status> {
        let crate::dt::SubmitEventRequest { user_token, event } = request.into_inner();

        let valid = db::validate_token(&self.db, &user_token)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if !valid {
            return Err(Status::unauthenticated("Invalid token"));
        }

        if let Some(event) = event {
            db::insert_event(&self.db, &event)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
        }

        let reply = SubmitEventResponse { success: true };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "dt_backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let addr = "[::1]:80".parse()?;
    let log_service = LogCollector::default();
    let db = db::new().await?;
    let dt_service = DtService::new(db);

    info!("WebService listening on {}", addr);

    let log_grpc_service = LogCollectorServiceServer::new(log_service);
    let log_grpc_web_service = tonic_web::enable(log_grpc_service);

    let dt_grpc_service = DtServer::new(dt_service);
    let dt_grpc_web_service = tonic_web::enable(dt_grpc_service);

    let static_files_service = any_service(ServeDir::new("../frontend/dist"));
    let media_files_service = any_service(ServeDir::new("media"));

    let app = Router::new()
        .nest_service("/grpc/log", log_grpc_web_service)
        .nest_service("/grpc/dt", dt_grpc_web_service)
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
