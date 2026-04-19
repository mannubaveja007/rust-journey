use reqwest::{self, Error};
use select::document::Document;
use select::predicate::Name;
use tokio;

#[tokio::main]

async fn main() -> Result<(), Error> {
    println!("Extracting information...");
    let res = reqwest::get("https://www.google.com").await?.text().await?;
    // test btw
    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
