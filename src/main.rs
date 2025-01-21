use uuid::Uuid;
use askama::Template;
use serde::{Serialize, Deserialize};
use actix_web::{web::{self, Form}, App, HttpResponse, HttpServer};
use anyhow::Result;

#[derive(Template)]
#[template(path = "queue.html")]
struct Queue {
    candidates: Vec<Candidate>
}

struct Candidate {
    id: String,
    name: String,
    score: f64,
}

#[derive(Debug, Deserialize)]
struct Vote {
    decision: f64,
    //TODO make to Uuid
    voter_id: String,
    candidate_id: String,
}

async fn queue() -> Queue {
    Queue {
        candidates: vec![
            Candidate {
                id: "xdd".into(),
                name: "fuh te".into(),
                score: 6.4
            },
            Candidate {
                id: "xde".into(),
                name: "ági fut-e".into(),
                score: 3.4
            }
        ]
    } 
}

//async fn vote(Path((voter, candidate)): Path<(String, String)>) -> Queue {
async fn vote(Form(req): Form<Vote>) -> Queue {
    println!("{req:#?}");
    Queue {
        candidates: vec![
            Candidate {
                id: "xdd".into(),
                name: "fuh te".into(),
                score: 6.4
            },
            Candidate {
                id: "xde".into(),
                name: "ági fut-e".into(),
                score: 3.4
            }
        ]
    } 
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("listening on 0.0.0.0:80");

    Ok(HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(queue))
            .route("/vote", web::post().to(vote))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await?)
}
