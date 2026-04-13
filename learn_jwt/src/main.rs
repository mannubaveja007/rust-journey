// new Cool thing!
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
// todo -> Generate JWT and explore it's library from docs.rs
use jsonwebtoken::{Algorithm, Header, encode};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // User ID
    email: String,
    exp: usize, // Expiration timestamp
}

fn main() {
    println!("Hello, world!");
    let val: Claims = Claims {
        sub: "bruhh".to_string(),
        email: "your_email@idk.io".to_string(),
        exp: 23476238432,
    };
    let token = encode(
        &Header::default(),
        &val,
        &EncodingKey::from_secret("Nhi_Bataungaa".as_ref()),
    )
    .expect("unable to generate token");

    println!("token => {}", token);
}
