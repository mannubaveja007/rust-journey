use reqwest::{self, Error};
use select::document::Document;
use select::predicate::Name;
use tokio;

#[tokio::main]

async fn main() -> Result<(),Error> {
    println!("Hello, world!");

    Ok(())
}
