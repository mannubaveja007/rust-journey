// new Cool thing!
use serde::{Deserialize, Serialize};

// todo -> Generate JWT and explore it's library from docs.rs

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // User ID
    email: String,
    exp: usize, // Expiration timestamp
}

fn main() {
    println!("Hello, world!");
}
