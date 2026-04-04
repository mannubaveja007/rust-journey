use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};

use redis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
// req / res structs
//

async fn some_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut conn = state.redis.lock().unwrap();
}

#[derive(Clone)]
struct AppState {
    redis: Arc<Mutex<redis::Connection>>,
}

#[tokio::main]
async fn main() {
    // 1. Connect to redis
    let client = redis::Client::open("redis_url").unwrap();
    let mut conn = client.get_connection().unwrap();

    // 2. Wrap in Arc<Mutext>> and put into the AppState
    //
    let state = AppState {
        redis: Arc::new(Mutex::new(conn)),
    };

    // 3. Pass to router
    //
    let app = Router::new()
        .route("/", get(some_handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
    // println!("Hello, world!");
}
