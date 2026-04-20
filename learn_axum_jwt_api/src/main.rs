use serde::{Serialize,Deserialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,   // User ID
    email: String,
    exp: usize,   // Expiration timestamp
}





fn main() {
    println!("Now We are going to start Axum API endpoint + JWT proper Scalable API!");
}
