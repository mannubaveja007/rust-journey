use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{self, Client, Request, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Formatter;

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
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

fn print_tracks(tracks: Vec<&Track>) {
    // to implemented this one later
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let token = &args[2];

    // spotify API to be used here
    //
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = query
    );
    let client = Client::new();
    let res = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .expect("there is some problem with reqesting...");

    match res.status() {
        StatusCode::OK => match res.json::<APIResponse>().await {
            Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
            Err(_) => println!("the response didn't match the shape we expected."),
        },
        StatusCode::UNAUTHORIZED => {
            println!("please enter new authentication token to continue!")
        }
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    }

    println!(
        "This is an project which will be using spotify API and it will be acting as an search system for spotify using their API!"
    );
}
