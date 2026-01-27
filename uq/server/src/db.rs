use anyhow::Result;
use sqlx::SqlitePool;
use crate::uq_proto::Event;

pub type Db = SqlitePool;

pub async fn connect(url: &str) -> Result<Db> {
    let pool = SqlitePool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub async fn insert_event(db: &Db, event: &Event) -> Result<()> {
    // Note: signature verification should happen before this.
    sqlx::query(
        "INSERT OR IGNORE INTO events (topic_pk, author_pk, signature, payload, server_timestamp_ms) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&event.topic_pk)
    .bind(&event.author_pk)
    .bind(&event.signature)
    .bind(&event.payload)
    .bind(event.server_timestamp_ms)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_events_since(db: &Db, since_ms: i64) -> Result<Vec<Event>> {
    let events = sqlx::query_as::<_, Event>(
        "SELECT * FROM events WHERE server_timestamp_ms > ? ORDER BY server_timestamp_ms ASC"
    )
    .bind(since_ms)
    .fetch_all(db)
    .await?;
    Ok(events)
}
