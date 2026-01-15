#![warn(missing_docs)]
//! Library for implementing the backend functions for the Democratic Tier service. 
use connectrpc_axum::{ConnectError, ConnectRequest, ConnectResponse};
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;

/// Generated crate from log protobuf definition by `buf`.
pub mod log_proto {
    include!(concat!(env!("OUT_DIR"), "/log.v1.rs"));
}

/// Generated crate from dt protobuf definition by `buf`.
pub mod dt_proto {
    include!(concat!(env!("OUT_DIR"), "/dt.v1.rs"));
}

use log_proto::{LogRequest, LogResponse, logcollectorservice};
use dt_proto::{SyncRequest, SyncResponse, dtservice};


/*
#[tracing::instrument]
*/
pub async fn log_connectrpc(ConnectRequest(req): ConnectRequest<LogRequest>) -> Result<ConnectResponse<LogResponse>, ConnectError> {
    info!("Client log: {}", req.message);

    Ok(ConnectResponse::new(LogResponse {}))
}

/// Struct for implementing the Democratic Tier service on top of the database.
#[derive(Debug)]
pub struct DtInstance {
    db: db::Db,
}

impl DtInstance {
    /// Creates new instance.
    pub fn new(db: db::Db) -> Self {
        Self { db }
    }
}

/*
pub async fn sync(&self, request: Request<SyncRequest>) -> Result<Response<SyncResponse>, Status> {
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

    let mut latest_timestamp = chrono::Utc::now().timestamp_millis();
    for event in &new_events {
        latest_timestamp = db::insert_event(&self.db, event)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
    }

    let reply = SyncResponse {
        events,
        server_timestamp_ms: latest_timestamp,
    };

    Ok(ConnectResponse::new(reply))
}
*/

/// Initializes logging for both stdout in easy to read format, and rotating log files as `jsonl` for
/// processing with monitoring tools.
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
