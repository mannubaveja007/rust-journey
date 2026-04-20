use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation, decode};
use jsonwebtoken::{EncodingKey, Header, encode};

use serde::{Deserialize, Serialize};
use axum::{Router, routing::get};
use axum::{
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, 
    email: String,
    exp: usize,
}

const JWT_SECRET: &[u8] = b"THIS_IS_VERY_VERY_STRONG_PASSWORD";

fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(data.claims)
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

   

    // encode is the main function
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .unwrap()
}

async fn login_handler() -> String {
    create_jwt("123".to_string(), "user@example.com".to_string())
}

async fn auth_middleware(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let token = bearer.token();
    match verify_jwt(token) {
        Ok(_) => Ok(next.run(request).await),
        Err(_) => Err(axum::http::StatusCode::UNAUTHORIZED),
    }
}

async fn protected_handler() -> &'static str {
    "Bruhhh Protected data accessed!"
}


#[tokio::main]
async fn main() {
    println!("Now We are going to start Axum API endpoint + JWT proper Scalable API!");
    println!("Listening on port : http://127.0.0.1:8080");
    let protected = Router::new()
        .route("/protected", get(protected_handler))
        .layer(axum::middleware::from_fn(auth_middleware));

    let app = Router::new()
        .route("/login", get(login_handler))
        .merge(protected);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
