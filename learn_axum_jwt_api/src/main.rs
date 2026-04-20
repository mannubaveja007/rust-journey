use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation, decode};
use jsonwebtoken::{EncodingKey, Header, encode};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // User ID
    email: String,
    exp: usize, // Expiration timestamp
}

fn verify_jwt(token: &str) -> Claims {
    let secret = b"SUPER_SECRET_KEY";

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .unwrap();
    data.claims
}

fn create_jwt(user_id: String, email: String) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    // real value
    let claims = Claims {
        sub: user_id,
        email,
        exp: expiration,
    };

   

    let secret = b"THIS_IS_VERY_VERY_STRONG_PASSWORD";
    // encode is the main function
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}

fn main() {
    println!("Now We are going to start Axum API endpoint + JWT proper Scalable API!");
}
