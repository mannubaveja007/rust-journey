use axum::{Json, Router, body::Body, routing::get,extract::State};
use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};
use tokio;
use std::sync::Arc;
use tokio::net::TcpListener;


#[derive(Clone)]
struct AppState{
    client : Client,
}

async fn get_stocks(State(state) : State<Arc<AppState>>) -> Result<Json<Document>, String> {
    let db = state.client.database("stockDB");
    let my_coll: Collection<Document> = db.collection("stocks");

    match my_coll.find_one(doc! {"name": "Tesla"}).await {
            Ok(Some(doc)) => Ok(Json(doc)),
            Ok(None) => Err("Stock not found".into()),
            Err(e) => Err(e.to_string()),
        }
}

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Connecting to the database...");
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;

    let state = Arc::new(AppState { client});
    let app = Router::new()
        .route("/", get(get_stocks))
        .with_state(state);

    let listner = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
    Ok(())
}
