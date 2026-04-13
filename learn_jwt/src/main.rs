// new Cool thing!
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode, errors::Error};
use serde::{Deserialize, Serialize};
use std::io;

use jsonwebtoken::{Algorithm, Header, encode};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String, // User ID
    email: String,
    exp: usize, // Expiration timestamp
}

fn gen_jwt(email: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    let val: Claims = Claims {
        sub: "bruhh".to_string(),
        email: email.to_string(),
        exp: 23476238432,
    };
    let secret = b"PUT_YOUR_OWN_SECRET_HERE";
    let tkn = encode(&Header::default(), &val, &EncodingKey::from_secret(secret))
        .expect("nope unable to generate JWT token");
    return tkn;
}

fn decode_jwt(token: String, secret: String) -> Result<(), Error> {
    let bruh: jsonwebtoken::TokenData<Claims> = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .expect("error");
    println!("{:?}", bruh);
    Ok(())
}
fn main() {
    let mut email = String::from("");
    println!("Enter your email Mr!");
    io::stdin().read_line(&mut email).unwrap();
    let token = gen_jwt(email.trim());

    println!("token => {}", token);
}
