use actix_files::NamedFile;
use askama::Template;
use serde::Deserialize;
use actix_web::{Error, web::{self, Form}, App, HttpServer};
use anyhow::{anyhow, bail, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

const MUSIC_DIRECTORY: &str = "./music";

#[derive(Template)]
#[template(path = "host.html")]
struct Host {
    songs: Vec<Song>
}

#[derive(Template)]
#[template(path = "queue.html")]
struct Queue {
    candidates: Vec<Candidate>
}

#[derive(Clone, Debug)]
struct Song {
    id: String,
    name: String,
}

struct Candidate {
    id: String,
    name: String,
    score: f64,
}

#[derive(Debug, Deserialize)]
struct Vote {
    decision: i8,
    voter_id: String, // TODO refactor to UUID
    candidate_id: String,
}

struct AppState {
    music_dir: PathBuf,
}

async fn get_songs_from_dir(music_dir: &Path) -> Result<Vec<Song>> {
    let mut songs = Vec::new();
    let mut entries = fs::read_dir(music_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(filename_os) = path.file_name() {
                if let Some(filename_str) = filename_os.to_str() {
                    if filename_str.ends_with(".mp3") || filename_str.ends_with(".ogg") || filename_str.ends_with(".wav") || filename_str.ends_with(".m4a") {
                        let id = filename_str.into();
                        let name = path.file_stem()
                            .ok_or_else(|| anyhow!("Failed to extract file stem for {filename_str}"))?
                            .to_str()
                            .unwrap()
                            .into();
                        songs.push(Song {
                            // Use filename as ID, URL encode if needed later for robustness
                            id,
                            // Use filename as name, can parse metadata later if desired
                            name,
                        });
                    }
                }
            }
        }
    }
    // Sort songs alphabetically by name for consistent listing
    songs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(songs)
}


// --- Handlers ---

// Serves the host page
async fn host_page(data: web::Data<AppState>) -> Result<Host, Error> {
    let songs = get_songs_from_dir(&data.music_dir)
        .await
        .map_err(|e| {
            log::error!("Failed to read music directory: {}", e);
            actix_web::error::ErrorInternalServerError("Could not load songs")
        })?;

    Ok(Host { songs })
}

// Serves a specific song file
async fn serve_song(
    data: web::Data<AppState>,
    song_id: web::Path<String>,
) -> Result<NamedFile, Error> {
    let song_filename = song_id.into_inner();
    // Basic sanitization: prevent directory traversal
    // This still assumes filenames don't contain problematic chars like '..'.
    // Using UUIDs mapped to paths would be more robust later.
    if song_filename.contains('/') || song_filename.contains('\\') || song_filename.contains("..") {
         return Err(actix_web::error::ErrorBadRequest("Invalid song ID"));
    }

    let file_path = data.music_dir.join(&song_filename);

    log::info!("Attempting to serve file: {:?}", file_path); // Added logging

    NamedFile::open(file_path)
        .map_err(|e| {
            log::error!("Failed to open file {:?}: {}", song_filename, e); // Added logging
            match e.kind() {
                std::io::ErrorKind::NotFound => actix_web::error::ErrorNotFound("Song not found"),
                _ => actix_web::error::ErrorInternalServerError("Error serving file"),
            }
        })
}


// Placeholder for the voting queue page (will be filled later)
async fn queue() -> Queue {
     // TODO: Fetch real candidates from DB later
    Queue {
        candidates: vec![
            Candidate {
                id: "placeholder1".into(),
                name: "Song A (Placeholder)".into(),
                score: 1.0
            },
            Candidate {
                id: "placeholder2".into(),
                name: "Song B (Placeholder)".into(),
                score: 0.0
            }
        ]
    }
}

// Placeholder for handling votes (will be filled later)
async fn vote(Form(req): Form<Vote>) -> Queue {
    println!("Received vote: {req:#?}");
    // TODO: Process vote (store in DB, recalculate scores)
    // TODO: Return updated queue fragment
    queue().await // Just return the static queue for now
}


// --- Main Function ---
#[tokio::main]
async fn main() -> Result<()> {
    // Basic logging setup
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let music_dir = PathBuf::from(MUSIC_DIRECTORY);
    if !music_dir.exists() {
        log::error!("Music directory does not exist: {:?}", music_dir);
        bail!("Music directory '{}' not found!", MUSIC_DIRECTORY);
    } else if !music_dir.is_dir() {
        bail!("Path '{}' is not a directory!", MUSIC_DIRECTORY);
    } else {
        log::info!("Serving music from: {:?}", music_dir.canonicalize()?);
    }


    const ADDR: &str = if cfg!(feature="dev") {"0.0.0.0:1337"} else {"0.0.0.0:80"};
    log::info!("Listening on {}", ADDR);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                music_dir: music_dir.clone(), // Clone PathBuf for thread safety
            }))
            .route("/", web::get().to(queue)) // Voting page (placeholder)
            .route("/vote", web::post().to(vote)) // Voting action (placeholder)
            .route("/host", web::get().to(host_page)) // Host control page
            .route("/play/{song_id}", web::get().to(serve_song)) // Serve individual songs
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind(ADDR)?
    .run()
    .await?;

    Ok(())
}

