#![warn(missing_docs)]
//! Library for implementing the backend functions for the Democratic Tier service.
use axum::{Router, routing::any_service};
use tower_http::services::ServeDir;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use uq::UqServer;

/// Service that serves static files from the frontend distribution directory.
pub fn static_files_service() -> Router {
    Router::new().fallback_service(any_service(ServeDir::new("../frontend/dist")))
}

/// Initializes logging for both stdout in easy to read format, and rotating log files as `jsonl` for
/// processing with monitoring tools.
pub fn init_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly("logs", "dt_log.jsonl");
    let (log_writer, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or("dt=debug,tower_http=debug,uq=debug,client_logs=info".into()),
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
