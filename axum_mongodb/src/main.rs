use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio;


#[derive(Debug, Deserialize, Serialize)]
struct GetDataResponse {
    text: String,
}

async fn plain_text() -> &'static str {
    "Hello World endpoint!"
}

async fn echo_response(
    Path(text): Path<String>,
    Json(body): Json<GetDataResponse>,
) -> impl IntoResponse {
    Json(GetDataResponse {
        text: format!("{} ", body.text),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(plain_text))
        .route("/echo/{text}", post(echo_response));

    println!(
        "In this we are going to use axum and build and simple todolist and connect it with mongodb. Looks Good!!"
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server is running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
