use axum::{extract::State, routing::any_service, Router};
use connectrpc_axum::{ConnectError, ConnectRequest, ConnectResponse};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;

/// Generated crate from log protobuf definition by `buf`.
#[allow(missing_docs)]
pub mod log_proto {
    include!(concat!(env!("OUT_DIR"), "/log.v1.rs"));
}

/// Generated crate from dt protobuf definition by `buf`.
#[allow(missing_docs)]
pub mod dt_proto {
    include!(concat!(env!("OUT_DIR"), "/dt.v1.rs"));
}

use dt_proto::{dtservice, SyncRequest, SyncResponse};
use log_proto::{logcollectorservice, LogRequest, LogResponse};

/// Returns the ConnectRPC router for the log collector service.
pub fn connect_router(db: db::Db) -> Router {
    let dt_router = dtservice::DtServiceBuilder::new()
        .sync(sync)
        .with_state(db)
        .build_connect();

    let log_router = logcollectorservice::LogCollectorServiceBuilder::new()
        .log(collect_log)
        .build_connect();

    Router::new().merge(log_router).merge(dt_router)
}

/// Returns a router that serves static files from the frontend build directory.
/// TODO DEV ONLY
pub fn static_files_service() -> Router {
    Router::new().fallback_service(any_service(ServeDir::new("../frontend/dist")))
}

#[tracing::instrument]
async fn collect_log(
    ConnectRequest(req): ConnectRequest<LogRequest>,
) -> Result<ConnectResponse<LogResponse>, ConnectError> {
    info!("Client log: {}", req.message);

    Ok(ConnectResponse::new(LogResponse {}))
}

#[tracing::instrument]
pub async fn sync(
    State(db): State<db::Db>,
    ConnectRequest(request): ConnectRequest<SyncRequest>,
) -> Result<ConnectResponse<SyncResponse>, ConnectError> {
    let SyncRequest {
        new_events,
        since_timestamp_ms,
    } = request;
    // For every database, we have the first event as the public key whitelist,
    // which as an unencrypted JSON array.
    // The first two events are signed with the first key of the whitelist.
    // Thus the server can validate incoming events, but cannot read the
    // second event, the manifest of the group, because it's encrypted
    // using the group key.

    // TODO crypograpy: check for the first two events, if they're invalid, abort.

    let events = db::get_events_since(&db, since_timestamp_ms)
        .await
        .map_err(|e| ConnectError::new(connectrpc_axum::Code::Internal, e.to_string()))?;

    let mut latest_timestamp = chrono::Utc::now().timestamp_millis();
    for event in &new_events {
        latest_timestamp = db::insert_event(&db, event)
            .await
            .map_err(|e| ConnectError::new(connectrpc_axum::Code::Internal, e.to_string()))?;
    }

    let reply = SyncResponse {
        events,
        server_timestamp_ms: latest_timestamp,
    };

    Ok(ConnectResponse::new(reply))
}

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
