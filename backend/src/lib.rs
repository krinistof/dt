use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
                .with_writer(log_writer)
        )
        .with(
            // for the stdout logs
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
        )
        .init();

    guard
}
