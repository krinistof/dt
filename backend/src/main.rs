use anyhow::Result;
use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response as AxumResponse},
    Router, routing::{any_service, get},
};
use lofty::{file::TaggedFileExt, probe::Probe};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tonic::{Request, Response, Status};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
#[cfg(feature = "scanner")]
mod scanner;

pub mod log {
    tonic::include_proto!("log.v1");
}

pub mod dt {
    tonic::include_proto!("dt.v1");
}

use log::{
    LogRequest, LogResponse,
    log_collector_service_server::{LogCollectorService, LogCollectorServiceServer},
};

use dt::{
    Event, GetInitialStateRequest, GetInitialStateResponse, Post, SubmitEventRequest,
    SubmitEventResponse, SyncEventsRequest, SyncEventsResponse,
    dt_server::{Dt, DtServer},
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

#[derive(Debug, Deserialize)]
struct PostPayload {
    content: String,
}

#[derive(Debug, Deserialize)]
struct VotePayload {
    content_hash: String,
    score: i32,
}

#[derive(Debug, Serialize)]
struct ScoreUpdatePayload {
    post_id: String,
    total_score: i64,
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
            match event.action.as_str() {
                "post" => {
                    let payload: PostPayload = serde_json::from_str(&event.payload)
                        .map_err(|e| Status::invalid_argument(e.to_string()))?;
                    let mut hasher = Sha256::new();
                    hasher.update(payload.content.as_bytes());
                    let content_hash = format!("{:x}", hasher.finalize());
                    db::insert_post(&self.db, &content_hash, &payload.content)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                    db::insert_event(&self.db, &event)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                }
                "vote" => {
                    let payload: VotePayload = serde_json::from_str(&event.payload)
                        .map_err(|e| Status::invalid_argument(e.to_string()))?;
                    db::insert_vote(&self.db, &payload.content_hash, &user_token, payload.score)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;

                    let total_score = db::get_total_score(&self.db, &payload.content_hash)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;

                    let score_update_payload = ScoreUpdatePayload {
                        post_id: payload.content_hash,
                        total_score,
                    };

                    let score_update_event = Event {
                        client_key: uuid::Uuid::new_v4().to_string(),
                        action: "score_update".to_string(),
                        payload: serde_json::to_string(&score_update_payload)
                            .map_err(|e| Status::internal(e.to_string()))?,
                        created_at: chrono::Utc::now().timestamp_millis(),
                    };

                    db::insert_event(&self.db, &score_update_event)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                }
                _ => return Err(Status::invalid_argument("Unknown event action")),
            }
        }

        let reply = SubmitEventResponse { success: true };

        Ok(Response::new(reply))
    }

    #[tracing::instrument(skip(self))]
    async fn get_initial_state(
        &self,
        request: Request<GetInitialStateRequest>,
    ) -> Result<Response<GetInitialStateResponse>, Status> {
        let user_token = request.into_inner().user_token;

        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let posts_with_scores = db::get_posts(&mut *tx, &user_token)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let latest_timestamp = db::get_latest_event_timestamp(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let posts = posts_with_scores
            .into_iter()
            .map(|p| Post {
                post_id: p.post_id,
                content: p.content,
                last_updated: p.last_updated,
                base_score: p.base_score as i32,
                user_score: p.user_score.unwrap_or(0) as i32,
            })
            .collect();

        let reply = GetInitialStateResponse {
            posts,
            server_timestamp: latest_timestamp,
        };
        Ok(Response::new(reply))
    }

    #[tracing::instrument(skip(self))]
    async fn sync_events(
        &self,
        request: Request<SyncEventsRequest>,
    ) -> Result<Response<SyncEventsResponse>, Status> {
        let since_timestamp = request.into_inner().since_timestamp;
        let event_rows = db::get_events_since(&self.db, since_timestamp)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let events: Vec<Event> = event_rows
            .iter()
            .map(|row| Event {
                client_key: row.event_id.clone(),
                action: row.event_type.clone(),
                payload: row.payload.clone(),
                created_at: row.server_created_at,
            })
            .collect();

        let new_timestamp = event_rows
            .last()
            .map(|row| row.server_created_at)
            .unwrap_or(since_timestamp);

        let reply = SyncEventsResponse {
            events,
            server_timestamp: new_timestamp,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "dt=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let addr = "0.0.0.0:8080".parse()?;
    let log_service = LogCollector::default();
    let db = db::new().await?;
    let dt_service = DtService::new(db.clone());

    #[cfg(feature = "scanner")]
    tokio::spawn(async move {
        if let Err(e) = scanner::scan_media_dir(&db).await {
            tracing::error!("Failed to scan media directory: {}", e);
        }
    });

    info!("WebService listening on {}", addr);

    let log_grpc_service = LogCollectorServiceServer::new(log_service);
    let log_grpc_web_service = tonic_web::enable(log_grpc_service);

    let dt_grpc_service = DtServer::new(dt_service);
    let dt_grpc_web_service = tonic_web::enable(dt_grpc_service);

    let static_files_service = any_service(ServeDir::new("../frontend/dist"));
    let media_files_service = any_service(ServeDir::new("media"));

    let app = Router::new()
        .route("/thumbnail/:filename", get(thumbnail_handler))
        .nest_service("/grpc/log", log_grpc_web_service)
        .nest_service("/grpc/dt", dt_grpc_web_service)
        .nest_service("/media", media_files_service)
        .fallback(static_files_service);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[tracing::instrument]
async fn thumbnail_handler(Path(filename): Path<String>) -> AxumResponse {
    let path_str = format!("./media/{}", filename);
    let path = std::path::Path::new(&path_str);

    if !path.exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    let tagged_file = match Probe::open(path) {
        Ok(probe) => match probe.read() {
            Ok(tagged_file) => tagged_file,
            Err(e) => {
                tracing::error!("Failed to read tags from file {:?}: {}", path, e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file").into_response();
            }
        },
        Err(e) => {
            tracing::error!("Failed to open file {:?}: {}", path, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to open file").into_response();
        }
    };

    if let Some((data, mime_type)) = tagged_file
        .primary_tag()
        .and_then(|t| t.pictures().get(0))
        .and_then(|p| p.mime_type().map(|m| (p.data(), m.to_string())))
    {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, mime_type)],
            data.to_vec(),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND, "No thumbnail found").into_response()
    }
}
