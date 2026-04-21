use reqwest;
use reqwest::header::{AUTHORIZATION,USER_AGENT,ACCEPT,CONTENT_TYPE};
use serde::{Deserialize,Serialize};


#[tokio::main]
async fn main() {
    println!(
        "This is an project which will be using spotify API and it will be acting as an search system for spotify using their API!"
    );
}
