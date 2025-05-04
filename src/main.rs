use actix_files::NamedFile;
use actix_web::{Error, web::{self, Form}, App, HttpServer, Responder, HttpResponse};
use anyhow::{bail, Context, Result};
use askama::Template;
use askama_actix::TemplateToResponse;
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

#[derive(Clone, Debug, sqlx::FromRow)]
struct Song {
    id: String,
    name: String,
    #[sqlx(skip)]
    file_path: Option<String>,
}

// Candidate represents a song in the voting queue
#[derive(Clone, Debug, sqlx::FromRow)]
struct Candidate {
    id: String, // Use the filename ID consistent with Song
    name: String,
    #[sqlx(default)] // Default score if not fetched directly via JOIN
    score: f32,
    #[sqlx(default)] // Default to None if the voter hasn't voted for this song
    voter_decision: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct Vote {
    decision: i8, // Keep this as i8 for the -127 to 127 range
    voter_id: Uuid, // Use Uuid type
    song_id: String, // This should match the Song's id
}

// Add the database pool to AppState
struct AppState {
    music_dir: PathBuf,
    db_pool: SqlitePool, // Add the database pool
}

// --- Database Functions ---

// Function to sync songs from the directory to the database
async fn sync_songs_to_db(music_dir: &Path, pool: &SqlitePool) -> Result<()> {
    log::info!("Starting song sync from directory: {:?}", music_dir);
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
                    if filename_str.ends_with(".mp3") || filename_str.ends_with(".ogg") || filename_str.ends_with(".wav") || filename_str.ends_with(".m4a") {
                        songs_found_in_dir += 1;
                        let id = filename_str.to_string();
                        let name = path.file_stem()
                            .unwrap_or_default() // Handle potential panic
                            .to_str()
                            .unwrap_or(&id) // Fallback to id if stem fails
                            .to_string();

                        // If song is not in DB, insert it
                        if !songs_in_db.contains(&id) {
                            log::info!("Adding new song to DB: ID={}, Name={}", id, name);
                            let full_path_str = path.canonicalize()?.to_string_lossy().to_string(); // Store canonical path

                            sqlx::query!(
                                "INSERT INTO songs (id, name, file_path) VALUES (?, ?, ?)",
                                id, name, full_path_str
                            )
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
        log::warn!("Removing song from DB (not found in directory): {}", missing_song_id);
        sqlx::query!("DELETE FROM songs WHERE id = ?", missing_song_id)
            .execute(pool)
            .await?;
         // Consider also deleting votes for this song? Or keep them? Depends on desired behavior.
        // sqlx::query!("DELETE FROM votes WHERE song_id = ?", missing_song_id).execute(pool).await?;
    }

    log::info!("Song sync completed. Found {} songs in dir, Added {} new songs.", songs_found_in_dir, songs_added);
    Ok(())
}

// Function to get songs (now reads from DB)
async fn get_songs_from_db(pool: &SqlitePool) -> Result<Vec<Song>> {
    let songs = sqlx::query_as!(
            Song,
            "SELECT id as id, name, file_path FROM songs ORDER BY name ASC"
        )
        .fetch_all(pool)
        .await?;
    Ok(songs)
}

// --- Handlers ---

// Serves the host page (reads songs from DB)
async fn host_page(data: web::Data<AppState>) -> Result<Host, Error> {
    let songs = get_songs_from_db(&data.db_pool)
        .await
        .map_err(|e| {
            log::error!("Failed to get songs from DB: {}", e);
            actix_web::error::ErrorInternalServerError("Could not load songs")
        })?;
    Ok(Host { songs })
}

// Serves a specific song file (reads path from DB)
async fn serve_song(
    data: web::Data<AppState>,
    song_id: web::Path<String>,
) -> Result<NamedFile, Error> {
    let song_filename = song_id.into_inner();

    // Basic sanitization still good, although we fetch path from DB now
    if song_filename.contains('/') || song_filename.contains('\\') || song_filename.contains("..") {
         return Err(actix_web::error::ErrorBadRequest("Invalid song ID format"));
    }

    // Fetch the file path from the database using the song ID (filename)
    let song_record = sqlx::query!(
            "SELECT file_path FROM songs WHERE id = ?",
            song_filename
        )
        .fetch_optional(&data.db_pool)
        .await
        .map_err(|e| {
            log::error!("Database error fetching song path for ID {}: {}", song_filename, e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    match song_record {
        Some(record) => {
            let file_path_str = &record.file_path;
            let file_path = PathBuf::from(file_path_str);
            log::info!("Attempting to serve file: {:?}", file_path);

            NamedFile::open(file_path)
                .map_err(|e| {
                    log::error!("Failed to open file {} (path: {:?}): {}", song_filename, record.file_path, e);
                    match e.kind() {
                        std::io::ErrorKind::NotFound => actix_web::error::ErrorNotFound("Song file not found on disk"),
                        _ => actix_web::error::ErrorInternalServerError("Error serving file"),
                    }
                })
        }
        None => {
            log::warn!("Song ID {} not found in database", song_filename);
            Err(actix_web::error::ErrorNotFound("Song not found in database"))
        }
    }
}


// --- Voter ID Handling ---
// Simple Cookie-based Voter ID (replace with more robust method if needed)
const VOTER_ID_COOKIE: &str = "voter_id";

// Always ensures the voter ID cookie is set in the outgoing jar
fn ensure_voter_id_cookie(req: &actix_web::HttpRequest, cookies: &mut actix_web::cookie::CookieJar) -> Uuid {
    let voter_id = req.cookie(VOTER_ID_COOKIE)
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

// Struct for the partial template context
#[derive(Template)]
#[template(path = "partials/candidate_list.html")]
struct CandidateList {
    candidates: Vec<Candidate>,
    voter_id: Uuid,
}

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
            s.id as id,
            s.name,
            -- Define the score alias here
            COALESCE(CAST(ss.total_score AS REAL), 0.0) as "score: f32",
            vd.decision as "voter_decision: i64"
        FROM songs s
        LEFT JOIN SongScores ss ON s.id = ss.song_id
        LEFT JOIN VoterDecisions vd ON s.id = vd.song_id
        -- Use the defined alias 'score' directly in ORDER BY
        ORDER BY "score" DESC, s.name ASC;
        "#, // <--- FIX: Refer to the alias "score" explicitly
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
            let template = Queue { candidates, voter_id };
            let body = template.render().unwrap_or_else(|e| {
                log::error!("Template rendering error: {}", e);
                "Error rendering page".to_string()
            });

            // Build the response
            let mut response_builder = HttpResponse::Ok();

            // Apply all cookies added to the jar to the response builder
            for cookie in jar.delta() { // delta() iterates over cookies added/removed
                response_builder.cookie(cookie.clone());
            }

            response_builder.content_type("text/html; charset=utf-8").body(body)
        }
        Err(e) => {
            log::error!("Failed to get candidates: {}", e);
            HttpResponse::InternalServerError().body("Could not load queue")
        }
    }
}

// Handles votes and returns the updated candidate list partial
async fn vote(
    data: web::Data<AppState>,
    Form(vote_data): Form<Vote>,
) -> impl Responder {
    log::info!("Received vote: {:?}", vote_data);

    // Validate decision range (optional but good practice)
    if !(-127..=127).contains(&vote_data.decision) {
         // Log the error, but maybe don't crash the request? Or return Bad Request.
         log::warn!("Invalid decision value received: {}", vote_data.decision);
         // Decide how strict to be. For now, we proceed.
    }

    let voter_id = vote_data.voter_id;
    let voter_id_string = voter_id.to_string();
    // Insert or Update the vote in the database
    // Using UPSERT logic (ON CONFLICT DO UPDATE) for SQLite
    let result = sqlx::query!(
        r#"
        INSERT INTO votes (voter_id, song_id, decision)
        VALUES (?, ?, ?)
        ON CONFLICT(voter_id, song_id) DO UPDATE SET
            decision = excluded.decision,
            timestamp = CURRENT_TIMESTAMP
        "#,
        voter_id_string,
        vote_data.song_id,
        vote_data.decision // Store decision directly
    )
    .execute(&data.db_pool)
    .await;

    match result {
        Ok(_) => {
            log::info!("Vote recorded successfully for voter {} on song {}", vote_data.voter_id, vote_data.song_id);
            // After successful vote, fetch updated candidates and return the partial
            match get_candidates_with_scores(&data.db_pool, voter_id).await {
                Ok(candidates) => {
                    let partial = CandidateList { candidates, voter_id };
                    // Return the rendered partial template
                    partial.to_response()
                }
                Err(e) => {
                     log::error!("Failed to get candidates after voting: {}", e);
                     HttpResponse::InternalServerError().body("<p>Error updating queue</p>")
                }
            }
        }
        Err(e) => {
            log::error!("Failed to record vote: {}", e);
            // Return an error response (maybe just the error message, or the previous state?)
             HttpResponse::InternalServerError().body("<p>Error saving vote</p>")
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Ensure database directory exists
    let db_dir = Path::new("db");
    if !db_dir.exists() {
        fs::create_dir(db_dir).await?;
        log::info!("Created database directory: {:?}", db_dir);
    }

    log::info!("Connecting to database: {}", DATABASE_URL);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .context("Failed to connect to database")?;

    log::info!("Running database migrations...");
    sqlx::migrate!("./db/migrations")
        .run(&pool)
        .await
        .context("Failed to run database migrations")?;
    log::info!("Database migrations completed.");

    // Prepare music directory
    let music_dir = PathBuf::from(MUSIC_DIRECTORY);
    if !music_dir.exists() {
        log::warn!("Music directory does not exist: {:?}. Creating it.", music_dir);
        bail!("Music directory '{}' not found!", MUSIC_DIRECTORY);
    } else if !music_dir.is_dir() {
        bail!("Path '{}' is not a directory!", MUSIC_DIRECTORY);
    } else {
        log::info!("Serving music from: {:?}", music_dir.canonicalize()?);
    }

    // Sync songs from directory to database on startup
    sync_songs_to_db(&music_dir, &pool).await?;


    const ADDR: &str = if cfg!(feature="dev") {"0.0.0.0:1337"} else {"0.0.0.0:80"};
    log::info!("Listening on {}", ADDR);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(AppState {
                music_dir: music_dir.clone(),
                db_pool: pool.clone(),
            }))
            .route("/", web::get().to(queue_page))
            .route("/vote", web::post().to(vote))
            .route("/host", web::get().to(host_page))
            .route("/play/{song_id}", web::get().to(serve_song))
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind(ADDR)?
    .run()
    .await?;

    Ok(())
}
