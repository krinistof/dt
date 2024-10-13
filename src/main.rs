
use axum::{routing::{get, post}, Router, extract::Path};
use askama::Template;

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

async fn queue() -> Queue {
    Queue {
        candidates: vec![Candidate {
            id: "xdd".into(),
            name: "John Doe".into(),
            score: 5.4
        }]
    } 
}

//async fn vote(Path((voter, candidate)): Path<(String, String)>) -> Queue {
async fn vote(axum::extract::Json(req): axum::extract::Json<serde_json::Value>) -> Queue {
    println!("{req:#?}");
    Queue {
        candidates: vec![Candidate {
            id: "xdd".into(),
            name: "John Done".into(),
            score: 6.4
        }]
    } 
}

#[tokio::main]
async fn main() {
    // define routes
    let app = Router::new()
        .route("/", get(queue))
        .route("/vote", post(vote));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
