use axum::{Router, routing::get};
use tokio;

#[tokio::main]
async fn main() {
    println!("The backend is working on http://127.0.0.1:3000");
    let app = Router::new().route("/", get(|| async { "Hello the backend is working fine" }));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listner, app).await.unwrap();
}
