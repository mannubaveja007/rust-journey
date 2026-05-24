use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use tokio;
use std::time::Instant;
use std::sync::{Arc,Mutex};


#[derive(Debug, Deserialize, Serialize)]
struct Response {
    status: String,
    code: String,
}

async fn search() -> &'static str {
    "You have reached to the search Endpoint."
}

async fn health() -> impl IntoResponse {
    Json(Response {
        status: "OK".to_string(),
        code: "200".to_string(),
    })
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/search", get(search))
        .route("/", get(health));

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("unable to run on port 3000");
    println!("Server is running on http://127.0.0.1:3000");
    println!("\nYou are allowed to Visit '/' and '/search' endpoint only");
    axum::serve(listner, app).await.unwrap();
}
