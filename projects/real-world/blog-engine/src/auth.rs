use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use actix_web::HttpRequest;
use crate::models::Claims;

pub fn create_token(user_id: &str, username: &str, is_admin: bool, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        is_admin,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn extract_claims(req: &HttpRequest, secret: &str) -> Option<Claims> {
    let auth_header = req.headers().get("Authorization")?;
    let auth_str = auth_header.to_str().ok()?;
    
    if !auth_str.starts_with("Bearer ") {
        return None;
    }

    let token = &auth_str[7..];
    verify_token(token, secret).ok()
}

pub fn verify_admin(req: &HttpRequest, secret: &str) -> bool {
    extract_claims(req, secret)
        .map(|claims| claims.is_admin)
        .unwrap_or(false)
}
