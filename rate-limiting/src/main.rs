use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::middleware::from_fn_with_state;
use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone)]
struct AppState {
    buckets: Arc<Mutex<HashMap<String, TokenBucket>>>,
}

pub struct TokenBucketInner {
    token: f64,
    capacity: f64,
    refill_rate: f64,
    last_refill: Instant,
}

// FIX 1: TokenBucket must derive/implement Clone because AppState derives Clone
// and TokenBucket values are stored inside AppState's HashMap.
#[derive(Clone)]
pub struct TokenBucket {
    inner: Arc<Mutex<TokenBucketInner>>,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(TokenBucketInner {
                token: capacity,
                capacity,
                refill_rate,
                last_refill: Instant::now(),
            })),
        }
    }

    pub fn allow_request(&self) -> bool {
        let mut inner = self.inner.lock().unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(inner.last_refill).as_secs_f64();
        inner.token = (inner.token + elapsed * inner.refill_rate).min(inner.capacity);
        inner.last_refill = now;

        if inner.token >= 1.0 {
            inner.token -= 1.0;
            true
        } else {
            false
        }
    }
}

// Rate-limit middleware

async fn rate_limit(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // FIX 2: The MutexGuard must NOT be held across the `next.run(request).await`
    // point — MutexGuard is !Send and the future must be Send for Tokio.
    // Resolve allow/deny first, drop the guard, then act on the result.
    let allowed = {
        let mut buckets = state.buckets.lock().unwrap();
        let bucket = buckets
            .entry(ip)
            .or_insert_with(|| TokenBucket::new(10.0, 1.0));
        bucket.allow_request()
    };

    if !allowed {
        return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
    }

    next.run(request).await.into_response()
}

// Handlers

// FIX 3: Renamed `Response` → `HealthResponse` to avoid shadowing
// axum / http response types and causing confusing compiler errors.
#[derive(Debug, Deserialize, Serialize)]
struct HealthResponse {
    status: String,
    code: String,
}

async fn search() -> &'static str {
    "You have reached the search endpoint."
}

async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "OK".to_string(),
        code: "200".to_string(),
    })
}

// Entry point

#[tokio::main]
async fn main() {
    let state = AppState {
        buckets: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/search", get(search))
        .route("/", get(health))
        .layer(from_fn_with_state(state.clone(), rate_limit))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("unable to bind to port 3000");

    println!("Server is running on http://127.0.0.1:3000");
    println!("Endpoints: '/' and '/search'");

    axum::serve(listener, app).await.unwrap();
}

// have used AI to fix  some core bugs but understood why they have occured!
