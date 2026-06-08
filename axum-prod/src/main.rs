use axum::Router;
use axum::routing::{get, post};
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio;


// now it's State Management Turn to learn that
#[derive(Deserialize, Debug)]
struct CreateTaskResp {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    description: String,
}

// this is for some specific invalid Input error of wrong String
#[derive(Debug)]
enum AppError {
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        };
        (status, error_message).into_response()
    }
}

async fn Create_Task(
    Json(payload): Json<CreateTaskResp>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    if payload.title.is_empty() {
        return Err(AppError::InvalidInput("Title cannot be empty".to_string()));
    }

    let task = Task {
        id: 1,
        title: payload.title,
        description: payload.description,
    };

    Ok((StatusCode::CREATED, Json(task)))
}

async fn hello_world() -> &'static str {
    "Welcome to the TASK API!!!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/create", post(Create_Task));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on the port http://127.0.0.1:3000");
    let _ = axum::serve(listner, app).await.unwrap();
}
