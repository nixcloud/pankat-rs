use crate::config;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// User access levels in the system
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserLevel {
    Admin,
    User,
    Guest,
}

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub level: UserLevel,
    pub exp: u64,
}

/// Create a new JWT token for a user
///
/// # Arguments
/// * `user_id` - The user's ID
/// * `level` - The user's access level
///
/// # Returns
/// * `Result<String, jsonwebtoken::errors::Error>` - The JWT token if successful
pub fn create_token(
    user_id: String,
    level: UserLevel,
) -> Result<String, jsonwebtoken::errors::Error> {
    let cfg = config::Config::get();

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 86400; // 24 hours

    let claims = Claims {
        user_id,
        level,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_token.as_bytes()),
    )
}

/// Validate and decode a JWT token
///
/// # Arguments
/// * `token` - The JWT token to validate
///
/// # Returns
/// * `Result<TokenData<Claims>, jsonwebtoken::errors::Error>` - The decoded claims if valid
pub fn validate_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let cfg = config::Config::get();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_token.as_bytes()),
        &Validation::default(),
    )
}
