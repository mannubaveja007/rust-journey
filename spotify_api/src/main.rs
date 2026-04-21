use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_link: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name : String,
    artists : Vec<Artist>,
    external_urls : ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name : String,
    href : String,
    popularity : u32,
    album : Album,
    external_urls : ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks : Items<Track>,
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items : Vec<T>
}



#[tokio::main]
async fn main() {
    println!(
        "This is an project which will be using spotify API and it will be acting as an search system for spotify using their API!"
    );
}
