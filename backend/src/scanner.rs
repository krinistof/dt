#![cfg(feature = "scanner")]

use anyhow::Result;
use lofty::prelude::{Accessor, AudioFile};
use lofty::probe::Probe;
use lofty::file::TaggedFileExt;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::Path;
use tracing::{error, info};

use crate::db;

#[derive(Debug)]
struct MusicPost {
    title: String,
    artist: String,
    url: String,
    length: u32,
}

fn get_music_post(path: &Path) -> Result<MusicPost> {
    let tagged_file = Probe::open(path)?.read()?;
    let properties = tagged_file.properties();
    let duration_seconds = properties.duration().as_secs() as u32;

    let tag = tagged_file
        .primary_tag()
        .ok_or_else(|| anyhow::anyhow!("No primary tag found"))?;

    let title = tag
        .title()
        .as_deref()
        .unwrap_or("Unknown Title")
        .to_string();
    let artist = tag
        .artist()
        .as_deref()
        .unwrap_or("Unknown Artist")
        .to_string();

    let url = format!(
        "/media/{}",
        path.file_name()
            .ok_or_else(|| anyhow::anyhow!("Could not get file name"))?
            .to_string_lossy()
    );

    Ok(MusicPost {
        title,
        artist,
        url,
        length: duration_seconds,
    })
}

pub async fn scan_media_dir(db: &db::Db) -> Result<()> {
    info!("Starting media scan...");
    let paths = std::fs::read_dir("./media")?;

    for path in paths {
        let path = path?.path();
        if path.is_file() {
            match get_music_post(&path) {
                Ok(music_post) => {
                    let MusicPost {
                        title,
                        artist,
                        url,
                        length
                    } = music_post;
                    let content = json!({
                        "type": "music",
                        "title": title,
                        "artist": artist,
                        "url": url,
                        "length": length,
                    })
                    .to_string();

                    let mut hasher = Sha256::new();
                    hasher.update(content.as_bytes());
                    let content_hash = format!("{:x}", hasher.finalize());

                    match db::insert_post(db, &content_hash, &content).await {
                        Ok(_) => info!("Added post: {}", content_hash),
                        Err(e) => {
                            if e.to_string().contains("UNIQUE constraint failed") {
                                // This is expected if the post already exists, so we can ignore it.
                            } else {
                                error!("Failed to insert post: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to process file {:?}: {}", path, e);
                }
            }
        }
    }
    info!("Media scan finished.");
    Ok(())
}
