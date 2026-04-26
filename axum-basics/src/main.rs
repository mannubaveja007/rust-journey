// import axum library and all necessary stuff
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};

use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// this Arc is allows multiple threads to access the same data
// and this Mutex allows only one thread to access the data at a time
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<HashMap<String, bool>>>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Deserialize)]
struct CreateTodoRequest {
    todo: String,
    completed: bool,
}

#[derive(Serialize)]
struct TodoResponse {
    todo: String,
    completed: bool,
}

#[derive(Deserialize)]
struct UpdateTodoRequest {
    completed: bool,
}

// handler - every handler is a function that takes a request and returns a response
//
// GET /
async fn hello() -> &'static str {
    "Hello from ArrXUm! 🦀"
}

async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "OK",
        version: env!("CARGO_PKG_VERSION"),
    })
}

// to get the todo by id
async fn get_todo(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let db = state.db.lock().unwrap();

    match db.get(&id) {
        Some(completed) => Json(TodoResponse {
            todo: id.clone(),
            completed: *completed,
        })
        .into_response(),

        None => (StatusCode::NOT_FOUND, format!("Todo {id} not found ")).into_response(),
    }
}

// POST /users - create a new user
async fn create_todo(
    State(state): State<AppState>,
    Json(body): Json<CreateTodoRequest>,
) -> impl IntoResponse {
    // take user inputs and parse them into a UserResponse
    let id = format!("todo : {}", body.todo.to_lowercase());
    let mut db = state.db.lock().unwrap();
    db.insert(id.clone(), body.completed.clone());
    Json(TodoResponse {
        todo: body.todo,
        completed: body.completed,
    })
}

async fn update_todo(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(body): Json<UpdateTodoRequest>,
) -> impl IntoResponse {
    let mut db = state.db.lock().unwrap();
    if let Some(completed) = db.get_mut(&id) {
        *completed = body.completed;
    }
    Json(TodoResponse {
        todo: id.clone(),
        completed: body.completed,
    })
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("Connecting to MongoDB Client...");
    let client = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .expect("Your Connection String is incorrect!");
    let dbs = client.list_database_names().await.expect("unable to fetch DBs");
    println!("Successfully Connected :=> {:?}",dbs);
    let database = client.database("todo_list_rust");

    let state = AppState {
        db: Arc::new(Mutex::new(HashMap::new())),
    };
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/todo", post(create_todo))
        .route("/todo/{id}", get(get_todo))
        .route("/todo/{id}", put(update_todo))
        .with_state(state);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server is running on http://127.0.0.1:3000");
    axum::serve(listner, app).await.unwrap();
}
