use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("requesting an GET request to httpbin!");
    let body: RequestResponse = reqwest::get("http://ip-api.com/json/<Your_IP_ADDRESS>")
        .await?
        .json()
        .await?;
    println!("body = {body:#?}");
    Ok(())
}
