use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer, json};
use tokio;
#[derive(Deserialize, Debug)]
struct RequestResponse {
    status: String,
    country: String,
    countryCode: String,
    region: String,
    regionName: String,
    city: String,
    zip: String,
    lat: f64,
    lon: f64,
    timezone: String,
    isp: String,
    org: String,
    r#as: String,
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WelcomeResponse {
    title: String,
    body: String,
    userId: String,
    id: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("requesting an GET request to ip-api!");
    println!("\n");
    // Just an GET Request!
    let body: RequestResponse = reqwest::get("http://ip-api.com/json/24.48.0.1")
        .await?
        .json()
        .await?;
    println!("body = {body:#?}");

    // Let's Try Post Request!

    let mut map: HashMap<String, String> = HashMap::new(); // Types inferred from later usage
    //    title: 'foo',
    //    body: 'bar',
    //    userId: 1,
    map.insert("title".to_string(), "Jason".to_string());
    map.insert("body".to_string(), "Sweet".to_string());
    map.insert("userId".to_string(), "1".to_string());

    println!("This is your HashMap/Body : {map:?}");
    let client = reqwest::Client::new();
    let req1 = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&map)
        .send()
        .await?;

    let res1: WelcomeResponse = req1.json().await?;

    println!("Post Request response : {res1:?}");
    Ok(())
}
