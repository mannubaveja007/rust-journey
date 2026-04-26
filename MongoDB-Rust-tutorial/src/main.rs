use mongodb::{
    Client, Collection,
    bson::{Document, doc},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Connecting to the database...");
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;

    let db = client.database("stockDB");

    let my_coll: Collection<Document> = db.collection("stocks");

    let my_stocks = my_coll.find_one(doc! {"name" : "Tesla" }).await?;
    println!("My stock : \n{:#?} ", my_stocks);
    Ok(())
}
