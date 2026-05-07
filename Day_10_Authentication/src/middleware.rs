use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::auth::{Claims, SECRET};

pub async fn auth_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(header) = auth_header {

        if let Some(token) = header.strip_prefix("Bearer ") {

            let decoded = decode::<Claims>(
                token,
                &DecodingKey::from_secret(SECRET),
                &Validation::default(),
            );

            if decoded.is_ok() {
                return Ok(next.run(req).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn protected() -> String {
    "Protected Route 🚀".to_string()
}