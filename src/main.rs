use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder,
    web::{self, Form},
};
use anyhow::{Context, Result, bail};
use askama::Template;
use askama_actix::TemplateToResponse;
use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

const MUSIC_DIRECTORY: &str = "./music";
const DATABASE_URL: &str = "sqlite:db/votes.db";

#[derive(Template)]
#[template(path = "host.html")]
struct Host {
    songs: Vec<Song>,
}

#[derive(Template)]
#[template(path = "queue.html")]
struct Queue {
    candidates: Vec<Candidate>,
    voter_id: Uuid,
}

/*
#[derive(Template)]
#[template(path = "partials/candidate_card.html")]
struct CandidateCard {
    candidate: Candidate,
    voter_id: Uuid,
}
*/

#[derive(Template)]
#[template(path = "partials/candidate_list.html")]
struct CandidateList {
    candidates: Vec<Candidate>,
    voter_id: Uuid,
}

#[derive(Clone, Debug, sqlx::FromRow)]
struct Song {
    id: String,
    name: String,
    played_at: Option<NaiveDateTime>,
}

// Candidate represents a song in the voting queue
#[derive(Clone, Debug, sqlx::FromRow)]
struct Candidate {
    id: String, // Use the filename ID consistent with Song
    name: String,
    #[sqlx(default)] // Default to None if the voter hasn't voted for this song
    voter_decision: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
struct NextSongInfo {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Vote {
    decision: i8,
    voter_id: Uuid,
    song_id: String,
}

struct AppState {
    db_pool: SqlitePool,
}

impl Candidate {
    pub fn html_id_suffix(&self) -> String {
        self.id
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => c,
                _ => '_', // Replace any non-alphanumeric character with an underscore
            })
            .collect()
    }
}

// --- Database Functions ---

// Function to sync songs from the directory to the database
async fn sync_songs_to_db(music_dir: &Path, pool: &SqlitePool) -> Result<()> {
    log::info!("Starting song sync from directory: {music_dir:?}");
    let mut songs_in_db: std::collections::HashSet<String> = sqlx::query!("SELECT id FROM songs")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.id)
        .collect();

    let mut entries = fs::read_dir(music_dir).await?;
    let mut songs_found_in_dir = 0;
    let mut songs_added = 0;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(filename_os) = path.file_name() {
                if let Some(filename_str) = filename_os.to_str() {
                    if filename_str.ends_with(".mp3")
                        || filename_str.ends_with(".ogg")
                        || filename_str.ends_with(".wav")
                        || filename_str.ends_with(".m4a")
                    {
                        songs_found_in_dir += 1;
                        let id = filename_str.to_string();
                        let name = path
                            .file_stem()
                            .unwrap_or_default() // Handle potential panic
                            .to_str()
                            .unwrap_or(&id) // Fallback to id if stem fails
                            .to_string();

                        // If song is not in DB, insert it
                        if !songs_in_db.contains(&id) {
                            log::info!("Adding new song to DB: ID={id}, Name={name}");

                            sqlx::query!("INSERT INTO songs (id, name) VALUES (?, ?)", id, name)
                                .execute(pool)
                                .await?;
                            songs_added += 1;
                        } else {
                            // Remove from the set, remaining items will be deleted later
                            songs_in_db.remove(&id);
                        }
                    }
                }
            }
        }
    }

    // Remove songs from DB that are no longer in the directory
    for missing_song_id in songs_in_db {
        log::warn!("Removing song from DB (not found in directory): {missing_song_id}");
        sqlx::query!("DELETE FROM songs WHERE id = ?", missing_song_id)
            .execute(pool)
            .await?;
        // TODO create test: does the cascading delete removes votes? if not:
        // sqlx::query!("DELETE FROM votes WHERE song_id = ?", missing_song_id).execute(pool).await?;
    }

    log::info!(
        "Song sync completed. Found {songs_found_in_dir} songs in dir, Added {songs_added} new songs."
    );
    Ok(())
}

// Function to get songs (now reads from DB)
async fn get_songs_from_db(pool: &SqlitePool) -> Result<Vec<Song>> {
    let songs = sqlx::query_as!(Song, "SELECT * FROM songs")
        .fetch_all(pool)
        .await?;
    Ok(songs)
}

// --- Handlers ---

// Serves the host page (reads songs from DB)
async fn host_page(data: web::Data<AppState>) -> Result<Host, Error> {
    let songs = get_songs_from_db(&data.db_pool).await.map_err(|e| {
        log::error!("Failed to get songs from DB: {e}");
        actix_web::error::ErrorInternalServerError("Could not load songs")
    })?;
    Ok(Host { songs })
}

// Endpoint to get the next song for the host player
async fn next_song_handler(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let pool = &data.db_pool;

    // Start a transaction
    let mut tx = pool.begin().await.map_err(|e| {
        log::error!("Failed to begin transaction: {e}");
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    // Find the top-scoring song that hasn't been played (played_at IS NULL)
    let next_song_candidate = sqlx::query_as!(
        NextSongInfo,
        r#"
        SELECT
            s.id as "id!",
            s.name as "name!"
        FROM songs s
        LEFT JOIN (
            SELECT song_id, SUM(decision) as total_score
            FROM votes
            GROUP BY song_id
        ) v ON s.id = v.song_id
        WHERE s.played_at IS NULL  -- Only select songs that haven't been played
        ORDER BY COALESCE(v.total_score, 0) DESC
        LIMIT 1;
        "#
    )
    .fetch_optional(&mut *tx) // Use the transaction
    .await
    .map_err(|e| {
        log::error!("Failed to query next song: {e}");
        actix_web::error::ErrorInternalServerError("Database error finding next song")
    })?;

    match next_song_candidate {
        Some(song) => {
            log::debug!("Next song selected: ID={}, Name={}", song.id, song.name);
            let now = Utc::now();
            let update_result =
                sqlx::query!("UPDATE songs SET played_at = ? WHERE id = ?", now, song.id)
                    .execute(&mut *tx) // Use the transaction
                    .await;

            match update_result {
                Ok(_) => {
                    // Commit the transaction
                    tx.commit().await.map_err(|e| {
                        log::error!("Failed to commit transaction: {e}");
                        actix_web::error::ErrorInternalServerError(
                            "Database error saving play status",
                        )
                    })?;
                    log::info!("Marked song {} as played.", song.id);
                    Ok(HttpResponse::Ok().json(song)) // Return song info as JSON
                }
                Err(e) => {
                    log::error!("Failed to mark song {} as played: {}", song.id, e);
                    // Rollback implicitly handled by drop, but good practice to log
                    Err(actix_web::error::ErrorInternalServerError(
                        "Database error updating play status",
                    ))
                }
            }
        }
        None => {
            log::warn!("No unplayed songs found in the queue.");
            // No need to commit/rollback as nothing was changed
            Ok(HttpResponse::NotFound().body("No unplayed songs available"))
        }
    }
}

// --- Voter ID Handling ---
// Simple Cookie-based Voter ID (replace with more robust method if needed)
const VOTER_ID_COOKIE: &str = "voter_id";

// Always ensures the voter ID cookie is set in the outgoing jar
fn ensure_voter_id_cookie(
    req: &actix_web::HttpRequest,
    cookies: &mut actix_web::cookie::CookieJar,
) -> Uuid {
    let voter_id = req
        .cookie(VOTER_ID_COOKIE)
        .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
        .unwrap_or_else(Uuid::new_v4); // Determine the ID: get existing or create new

    // Always build the cookie to be set in the response
    let cookie = actix_web::cookie::Cookie::build(VOTER_ID_COOKIE, voter_id.to_string())
        .path("/")
        .secure(false) // Set to true if using HTTPS
        .http_only(true) // Good practice for non-JS accessed cookies
        .max_age(actix_web::cookie::time::Duration::days(365)) // Refresh expiry
        .finish();

    // Add (or replace) the cookie in the outgoing jar
    cookies.add(cookie);

    voter_id // Return the determined ID
}

// --- Queue & Voting Logic ---

async fn get_candidates_with_scores(pool: &SqlitePool, voter_id: Uuid) -> Result<Vec<Candidate>> {
    let voter_id_str = voter_id.to_string();

    let candidates = sqlx::query_as!(
        Candidate,
        r#"
        WITH SongScores AS (
            SELECT
                song_id,
                SUM(decision) as total_score
            FROM votes
            GROUP BY song_id
        ), VoterDecisions AS (
            SELECT
                song_id,
                decision
            FROM votes
            WHERE voter_id = ?
        )
        SELECT
            s.id as "id!",
            s.name as "name!",
            vd.decision as "voter_decision: i64"
        FROM songs s
        LEFT JOIN SongScores ss ON s.id = ss.song_id
        LEFT JOIN VoterDecisions vd ON s.id = vd.song_id
        WHERE s.played_at IS NULL
        ORDER BY COALESCE(CAST(ss.total_score AS REAL), 0.0) DESC;
        "#,
        voter_id_str
    )
    .fetch_all(pool)
    .await?;

    Ok(candidates)
}

// Serves the main voting queue page
async fn queue_page(req: actix_web::HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let mut jar = actix_web::cookie::CookieJar::new();
    // This function now ensures the jar has the cookie for the response
    let voter_id = ensure_voter_id_cookie(&req, &mut jar);

    match get_candidates_with_scores(&data.db_pool, voter_id).await {
        Ok(candidates) => {
            let template = Queue {
                candidates,
                voter_id,
            };
            let body = template.render().unwrap_or_else(|e| {
                log::error!("Template rendering error: {e}");
                "Error rendering page".to_string()
            });

            // Build the response
            let mut response_builder = HttpResponse::Ok();

            // Apply all cookies added to the jar to the response builder
            for cookie in jar.delta() {
                response_builder.cookie(cookie.clone());
            }

            response_builder
                .content_type("text/html; charset=utf-8")
                .body(body)
        }
        Err(e) => {
            log::error!("Failed to get candidates: {e}");
            HttpResponse::InternalServerError().body("Could not load queue")
        }
    }
}

// Handler specifically for fetching and returning the queue partial content
async fn queue_content_handler(
    req: actix_web::HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // 1. Get Voter ID from cookie (DO NOT try to set it here)
    let voter_id = req
        .cookie(VOTER_ID_COOKIE)
        .and_then(|cookie| Uuid::parse_str(cookie.value()).ok())
        .unwrap_or_else(|| {
            // This case should ideally not happen if the main page loaded correctly,
            // but handle it gracefully. Generating a new ID here might lead to
            // temporary inconsistencies if the user somehow lost the cookie mid-session.
            // Returning an error or an empty list might be alternatives.
            log::warn!("Voter ID cookie not found during queue refresh polling.");
            Uuid::new_v4() // Fallback: generate temporary ID for this request
        });

    // 2. Fetch candidates using the retrieved voter ID
    match get_candidates_with_scores(&data.db_pool, voter_id).await {
        Ok(candidates) => {
            //TODO investigate: should cookie jar be handled here as well?
            let partial = CandidateList {
                candidates,
                voter_id,
            };
            partial.to_response()
        }
        Err(e) => {
            log::error!("Failed to get candidates for polling refresh: {e}");
            HttpResponse::InternalServerError().body("<p>Error refreshing queue</p>")
        }
    }
}

// Handles votes and returns the updated candidate list partial
async fn vote(data: web::Data<AppState>, Form(vote_data): Form<Vote>) -> impl Responder {
    log::debug!("Received vote: {vote_data:?}");

    let Vote {
        decision,
        voter_id,
        song_id,
    } = vote_data;

    if !(-127..=127).contains(&decision) {
        log::warn!("Invalid decision value {decision} received from {voter_id}");
        return HttpResponse::BadRequest()
            .body(format!("<p>Error saving vote with decision {decision}</p>"));
    }

    let voter_id_string = voter_id.to_string();
    // Insert or Update the vote in the database
    let result = sqlx::query!(
        r#"
        INSERT INTO votes (voter_id, song_id, decision)
        VALUES (?, ?, ?)
        ON CONFLICT(voter_id, song_id) DO UPDATE SET
            decision = excluded.decision,
            timestamp = CURRENT_TIMESTAMP
        "#,
        voter_id_string,
        song_id,
        decision // Store decision directly
    )
    .execute(&data.db_pool)
    .await;

    match result {
        Ok(_) => {
            log::debug!("Vote recorded successfully for voter {voter_id} on song {song_id}");

            match get_candidates_with_scores(&data.db_pool, voter_id).await {
                Ok(updated_candidates) => CandidateList {
                    candidates: updated_candidates,
                    voter_id,
                }
                .to_response(),
                Err(e) => {
                    log::error!(
                        "Failed to get full candidate list after voting (voter {voter_id}): {e}"
                    );
                    // Return an error message intended for the list container
                    HttpResponse::InternalServerError()
                        .body("<p>Error refreshing the queue after vote.</p>")
                }
            }
        }
        Err(e) => {
            log::error!("Failed to record vote for song {song_id}: {e}");
            HttpResponse::InternalServerError().body("<p>Error saving vote</p>")
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_dir = Path::new("db");
    if !db_dir.exists() {
        fs::create_dir(db_dir).await?;
        log::info!("Created database directory: {db_dir:?}");
    }

    //let db_file = db_dir.join("votes.db");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .context(format!("Failed to connect to database {DATABASE_URL}"))?;

    sqlx::migrate!("./db/migrations")
        .run(&pool)
        .await
        .context("Failed to run database migrations")?;

    let music_dir = PathBuf::from(MUSIC_DIRECTORY);
    if !music_dir.exists() {
        log::warn!("Music directory does not exist: {music_dir:?}",);
        bail!("Music directory '{MUSIC_DIRECTORY}' not found!");
    } else if !music_dir.is_dir() {
        bail!("Path '{MUSIC_DIRECTORY}' is not a directory!");
    } else {
        log::info!("Serving music from: {:?}", music_dir.canonicalize()?);
    }

    sync_songs_to_db(&music_dir, &pool).await?;

    const ADDR: &str = if cfg!(feature = "dev") {
        "0.0.0.0:8080"
    } else {
        "0.0.0.0:80"
    };
    log::info!("Listening on {ADDR}");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(AppState {
                db_pool: pool.clone(),
            }))
            .route("/", web::get().to(queue_page))
            .route("/vote", web::post().to(vote))
            .route("/host", web::get().to(host_page))
            .route("/queue", web::get().to(queue_content_handler))
            .route("/next", web::get().to(next_song_handler))
            .service(actix_files::Files::new("/static", "./static"))
            .service(actix_files::Files::new("/songs", MUSIC_DIRECTORY))
    })
    .bind(ADDR)?
    .run()
    .await?;

    Ok(())
}
