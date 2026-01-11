//! Module for database implementations. For now I will stick to SQLite for simplicity, and to
//! achieve a single executable solution, but later this can be quickly ported to eg. PostgreSQL.
use anyhow::Result;
use sqlx::SqlitePool;

use crate::dt_proto::Event;

/// This needs to be updated for a new database implementation.
pub type Db = SqlitePool;
const DEFAULT_SQLITE_URL: &str = "sqlite://db/dt.db?mode=rwc";

/// Returns a new database handle already initialized.
pub async fn new() -> Result<Db> {
    // TODO: ensure db directory exists
    let db_url = std::env::var("DATABASE_URL").unwrap_or(DEFAULT_SQLITE_URL.into());
    let pool = SqlitePool::connect(&db_url).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

//TODO cryptography
/*
pub async fn validate_events(db: &Db, events: Vec<Event>) -> Result<()> {
    let Some(whitelist_event) = sqlx::query_as::<_, Event>(
        "SELECT * FROM event_queue
        ORDER BY server_timestamp_ms DESC
        LIMIT 1",
    )
    .fetch_optional(db)
    .await?
    else {
        bail!("Database has no events.")
    };

    todo!();

    Ok(())
}
*/

/// Returns the events happened since the the submitted last timestamp.
pub async fn get_events_since(db: &Db, timestamp: i64) -> Result<Vec<Event>> {
    let events = sqlx::query_as(
        "SELECT * FROM event_queue
        WHERE server_timestamp_ms > ?
        ORDER BY server_timestamp_ms ASC",
    )
    .bind(timestamp)
    .fetch_all(db)
    .await?;
    Ok(events)
}

/// Inserts a new event to the database. If the event already exists it logs a warning but ignores.
pub async fn insert_event(db: &Db, event: &Event) -> Result<i64> {
    let Event {
        pub_key,
        signature,
        event_blob,
        ..
    } = event;
    let timestamp = chrono::Utc::now().timestamp_millis();

    let result = sqlx::query(
        "INSERT OR IGNORE INTO event_queue (pub_key, signature, event_blob, server_timestamp_ms) VALUES (?, ?, ?, ?)",
    )
    .bind(pub_key)
    .bind(signature)
    .bind(event_blob)
    .bind(timestamp)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        //TODO format keys a bit prettier than dumping a Vec<u8> in Debug
        tracing::warn!(
            "Public key {pub_key:?} attempted to insert a duplicate event (signature {signature:?}), ignoring."
        );
    }

    Ok(timestamp)
}
