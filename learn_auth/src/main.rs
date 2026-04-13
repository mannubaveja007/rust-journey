use reqwest::Error;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
struct BasicAuthResponse {
    authenticated: bool,
    user: String,
}

fn main() -> Result<(), Error> {
    println!("Connecting for Auth!!");
    let req = Client::new();
    let res = req
        .get("https://httpbin.org/basic-auth/user/password")
        .basic_auth("user", Some("password"))
        .send()?;
    let res_Real: BasicAuthResponse = res.json()?;
    println!("Authenticated : {}", res_Real.authenticated);
    println!("User : {}", res_Real.user);
    Ok(())
}
