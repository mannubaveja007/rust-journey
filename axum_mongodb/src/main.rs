use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tokio;

async fn plain_text() -> &'static str {
    "Hello World endpoint!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(plain_text));
    println!(
        "In this we are going to use axum and build and simple todolist and connect it with mongodb. Looks Good!!"
    );
    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server is running on http://127.0.0.1:3000");
    axum::serve(listner, app).await.unwrap();
}
