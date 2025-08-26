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
    let result = sqlx::query(
        "INSERT OR IGNORE INTO global_event_queue (event_id, event_type, payload, server_created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&event.client_key)
    .bind(&event.action)
    .bind(&event.payload)
    .bind(chrono::Utc::now().timestamp())
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        tracing::warn!(event_id = event.client_key, "Attempted to insert a duplicate event, ignoring.");
    }

    Ok(())
}

pub async fn insert_post(db: &Db, post_id: &str, content: &str) -> Result<()> {
    let result = sqlx::query("INSERT OR IGNORE INTO posts (post_id, content, last_updated) VALUES (?, ?, ?)")
        .bind(post_id)
        .bind(content)
        .bind(chrono::Utc::now().timestamp())
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        tracing::warn!(post_id = post_id, "Attempted to insert a duplicate post, ignoring.");
    }

    Ok(())
}

pub async fn insert_vote(db: &Db, post_id: &str, user_token: &str, decision: i32) -> Result<()> {
    sqlx::query(
        "INSERT OR REPLACE INTO votes (post_id, user_token, decision, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(post_id)
    .bind(user_token)
    .bind(decision)
    .bind(chrono::Utc::now().timestamp())
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_total_score(db: &Db, post_id: &str) -> Result<i64> {
    let result: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(decision), 0) FROM votes WHERE post_id = ?")
        .bind(post_id)
        .fetch_one(db)
        .await?;
    Ok(result.0)
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostWithScore {
    pub post_id: String,
    pub content: String,
    pub last_updated: i64,
    pub base_score: i64,
    pub user_score: Option<i64>,
}

pub async fn get_posts<'e, E>(executor: E, user_token: &str) -> Result<Vec<PostWithScore>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let posts = sqlx::query_as::<_, PostWithScore>(
        r#"
        SELECT
            p.post_id,
            p.content,
            p.last_updated,
            COALESCE(SUM(v.decision), 0) - COALESCE(SUM(CASE WHEN v.user_token = ? THEN v.decision ELSE 0 END), 0) AS base_score,
            SUM(CASE WHEN v.user_token = ? THEN v.decision ELSE NULL END) AS user_score
        FROM
            posts p
        LEFT JOIN
            votes v ON p.post_id = v.post_id
        GROUP BY
            p.post_id
        "#,
    )
    .bind(user_token)
    .bind(user_token)
    .fetch_all(executor)
    .await?;
    Ok(posts)
}

#[derive(Debug, sqlx::FromRow)]
pub struct EventRow {
    pub event_id: String,
    pub event_type: String,
    pub payload: String,
    pub server_created_at: i64,
}

pub async fn get_events_since(db: &Db, timestamp: i64) -> Result<Vec<EventRow>> {
    let events = sqlx::query_as::<_, EventRow>(
        r#"
        SELECT
            event_id,
            event_type,
            payload,
            server_created_at
        FROM
            global_event_queue
        WHERE
            server_created_at > ?
        ORDER BY
            server_created_at ASC
        "#,
    )
    .bind(timestamp)
    .fetch_all(db)
    .await?;
    Ok(events)
}

pub async fn get_latest_event_timestamp<'e, E>(executor: E) -> Result<i64>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let result: (Option<i64>,) =
        sqlx::query_as("SELECT MAX(server_created_at) FROM global_event_queue")
            .fetch_one(executor)
            .await?;
    Ok(result.0.unwrap_or(0))
}
