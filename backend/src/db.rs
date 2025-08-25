use anyhow::Result;
use sqlx::SqlitePool;

use crate::dt::Event;

pub type Db = SqlitePool;

pub async fn new() -> Result<Db> {
    //TODO DEV ONLY
    let pool = SqlitePool::connect("sqlite:dt.db?mode=rwc").await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

pub async fn validate_token(db: &Db, token: &str) -> Result<bool> {
    let result = sqlx::query("SELECT 1 FROM tokens WHERE token = ?")
        .bind(token)
        .fetch_optional(db)
        .await?;
    Ok(result.is_some())
}

pub async fn insert_event(db: &Db, event: &Event) -> Result<()> {
    sqlx::query(
        "INSERT INTO global_event_queue (event_id, event_type, payload) VALUES (?, ?, ?)",
    )
    .bind(&event.client_key)
    .bind(&event.action)
    .bind(&event.payload)
    .execute(db)
    .await?;
    Ok(())
}
