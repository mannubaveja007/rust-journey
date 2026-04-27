use axum::{
    Json, Router,
    body::Body,
    extract::State,
    routing::{get, post},
};
use mongodb::{
    Client, Collection,
    bson::{Document, doc},
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize, Serialize)]
struct StockUsers {
    walletAddress: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeResponse {
    #[serde(rename = "_id")]
    id: Id,
    name: String,
    price: i64,
    available_quantity: i64,
    symbol: String,
    #[serde(rename = "__v")]
    v: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Id {
    #[serde(rename = "$oid")]
    oid: String,
}

#[derive(Clone)]
struct AppState {
    client: Client,
}

async fn get_stocks(State(state): State<Arc<AppState>>) -> Result<Json<WelcomeResponse>, String> {
    let db = state.client.database("stockDB");
    let my_coll: Collection<WelcomeResponse> = db.collection("stocks");

    match my_coll.find_one(doc! {"name": "Tesla"}).await {
        Ok(Some(doc)) => Ok(Json(doc)),
        Ok(None) => Err("Stock not found".into()),
        Err(e) => Err(e.to_string()),
    }
}

// beautiful working function
async fn post_stock(
    State(state): State<Arc<AppState>>,
    Json(input): Json<StockUsers>,
) -> Result<Json<InsertOneResult>, String> {
    let db = state.client.database("stockDB");
    let my_coll: Collection<StockUsers> = db.collection("users");
    let result = my_coll.insert_one(input).await.map_err(|e| e.to_string())?;
    Ok(Json(result))
}

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Connecting to the database...");
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;

    let state = Arc::new(AppState { client });
    let app = Router::new()
        .route("/", get(get_stocks))
        .route("/stock", post(post_stock))
        .with_state(state);
    println!("Backend in running on http://127.0.0.1:3000");
    let listner = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
    Ok(())
}
