#![warn(missing_docs)]
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;
use tonic::{Request, Response, Status};

pub mod db;

pub mod log_proto {
    tonic::include_proto!("log.v1");
}

pub mod dt_proto {
    tonic::include_proto!("dt.v1");
}

use log_proto::{
    LogRequest, LogResponse,
    log_collector_service_server::LogCollectorService,
};

use dt_proto::{
    SyncRequest, SyncResponse,
    dt_server::Dt,
};

#[derive(Debug, Default)]
pub struct LogCollector {}

#[tonic::async_trait]
impl LogCollectorService for LogCollector {
    #[tracing::instrument]
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogResponse>, Status> {
        info!("Log: {}", request.get_ref().message);

        Ok(Response::new(LogResponse {}))
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
    async fn sync(&self, request: Request<SyncRequest>) -> Result<Response<SyncResponse>, Status> {
        let SyncRequest {
            new_events,
            since_timestamp_ms,
        } = request.into_inner();
        // For every database, we have the first event as the public key whitelist,
        // which as an unencrypted JSON array.
        // The first two events are signed with the first key of the whitelist.
        // Thus the server can validate incoming events, but cannot read the
        // second event, the manifest of the group, because it's encrypted
        // using the group key.

        // TODO crypograpy: check for the first two events, if they're invalid, abort.

        let events = db::get_events_since(&self.db, since_timestamp_ms)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut latest_timestamp = 0;
        for event in &new_events {
            latest_timestamp = db::insert_event(&self.db, event)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
        }

        let reply = SyncResponse {
            events,
            server_timestamp_ms: latest_timestamp,
        };

        Ok(Response::new(reply))
    }
}

pub fn init_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly("logs", "dt_log.jsonl");
    let (log_writer, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or("dt=debug,tower_http=debug".into()),
        ))
        .with(
            // for the log file to be processed later
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(log_writer),
        )
        .with(
            // for the stdout logs
            tracing_subscriber::fmt::layer().with_ansi(true),
        )
        .init();

    guard
}
