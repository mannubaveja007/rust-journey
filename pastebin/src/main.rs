use axum::extract::Request;
use axum::middleware;
use axum::middleware::Next;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};

use redis;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Deserialize)]
struct CreatePasteRequest {
    content:    String,          // the text content of the paste
    language:   Option<String>,  // e.g. "rust", "python" (optional)
    expires_in: Option<u64>,     // TTL in seconds, default 600 (10 mins)
}

#[derive(Serialize)]
struct PasteResponse {
    code:       String,   // short code e.g. "a1b2c3d4"
    content:    String,   // the paste text
    language:   String,   // programming language label
    views:      i64,      // how many times it has been viewed
    expires_in: i64,      // remaining TTL in seconds (-1 = no expiry)
}

#[derive(Serialize)]
struct CreatePasteResponse {
    code:     String,   // short code to share
    url:      String,   // full URL e.g. "http://localhost:3000/paste/a1b2c3d4"
}

#[derive(Clone)]
struct AppState {
    redis: redis::aio::MultiplexedConnection,
}


// ---- post /create Create a new paste -----------------

async fn create_paste(
    State(mut state) : State<AppState>,
    Json(body): Json<CreatePasteRequest>,
) -> impl IntoResponse {

    let code = Uuid::new_v4().to_string()[..8].to_string();
    
    let language = body.language.unwrap_or("text".to_string());
    
    let ttl = body.expires_in.unwrap_or(600);
    
    let _: () = state.redis.hset_multiple(&code, &[
        ("content",  &body.content),
        ("language", &language),
        ("views",    &String::from("0")),
    ]).await.unwrap();
    
    let _: () = state.redis.expire(&code, ttl.try_into().unwrap()).await.unwrap();
    
    let url = format!("http://localhost:3000/paste/{}", code);
    
    Json(CreatePasteResponse {
        code,
        url,
    }).into_response()
}

// --- Get /paste/{Code} Get a paste by short code-----------------

async fn get_paste(
    State(mut state) : State<AppState>,
    Path(code) : Path<String>,
) -> impl IntoResponse {
    // Returns a HashMap<String, String>


    let fields: std::collections::HashMap<String, String> =
        state.redis.hgetall(format!("paste:{}", code)).await.expect("ERROR");
    let content  = fields.get("content").cloned().unwrap_or_default();
    let language = fields.get("language").cloned().unwrap_or_default();
    let views    = fields.get("views").and_then(|v| v.parse::<i64>().ok()).unwrap_or(0);
    let ttl: i64 = redis::cmd("TTL")
        .arg(format!("paste:{}", code))
        .query_async(&mut state.redis)
        .await
        .unwrap();


        
    Json(PasteResponse{
        code,
        content,
        language,
        views,
        expires_in: ttl,
    }).into_response()   
}

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let conn = client.get_multiplexed_async_connection().await.unwrap();
    let app_state = AppState {
        redis: conn,
    };
    let app = Router::new()
    .route("/create", post(create_paste))
    .route("/paste/{code}", get(get_paste))
    .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
