use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post, put},
};

use redis;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
// req / res structs
//

async fn some_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut conn = state.redis;
}

// state to make it thread safe  no need of Arc<Mutex> cuz it's MultiplexedConnection is already thread safe
#[derive(Clone)]
struct AppState {
    redis: redis::aio::MultiplexedConnection,
}

// structs
//
#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    url: String,
    code: String,
}
// --- shorten url functionality -------------------
async fn shorten_url(
    State(mut state): State<AppState>,
    Json(body): Json<ShortenRequest>,
) -> impl IntoResponse {
    let url = body.url.clone();
    let code: String = Uuid::new_v4().to_string()[..8].to_string();
    let _: () = state.redis.set_ex(&code, &url, 3600).await.unwrap();
    return Json(ShortenResponse { url, code });
}

// --- redirect functionality -------------------
async fn redirect(
    State(mut state): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let url: Option<String> = state.redis.get(&code).await.unwrap();
    match url {
        Some(url) => Redirect::to(&url).into_response(),
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn show_all(State(mut state): State<AppState>) -> impl IntoResponse {
    let urls: Vec<String> = state.redis.keys("*").await.unwrap();
    Json(urls).into_response()
}

// --- Main ----------------------
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // 1. Connect to redis
    let client = redis::Client::open("redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320").unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();

    // 2. Wrap in Arc<Mutext>> and put into the AppState
    //
    let state = AppState { redis: conn };

    // 3. Pass to router

    let app = Router::new()
        .route("/shorten", post(shorten_url))
        .route("/{code}", get(redirect))
        .route("/urls", get(show_all))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
    // println!("Hello, world!");
}
