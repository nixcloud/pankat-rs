use crate::auth::{create_token, validate_token, UserLevel};
use crate::db::{create_user, get_user_by_username};
use crate::error::AppError;
use axum::{
    extract::State,
    http::header::{HeaderMap, AUTHORIZATION},
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// Request payload for registration and login
#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

/// Response containing the JWT token
#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

/// Register a new user
///
/// curl -X POST http://localhost:5000/api/auth/register -H "Content-Type: application/json" -d '{"username":"testuser","password":"testpass"}'
pub async fn register(
    State(pool): State<DbPool>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

    // Hash password
    let hashed =
        hash(req.password.as_bytes(), DEFAULT_COST).map_err(|_| AppError::InternalError)?;

    // Create user with default User level
    let user_id = create_user(&mut conn, &req.username, &hashed, UserLevel::User)
        .map_err(AppError::DatabaseError)?;

    // Generate token
    let token =
        create_token(user_id.to_string(), UserLevel::User).map_err(|_| AppError::InternalError)?;

    Ok(Json(AuthResponse { token }))
}

/// Login an existing user
///
/// curl -X POST http://localhost:5000/api/auth/login \
///   -H "Content-Type: application/json" \
///   -d '{"username":"testuser","password":"testpass"}'
pub async fn login(
    State(pool): State<DbPool>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

    // Get user
    let user = get_user_by_username(&mut conn, &req.username)
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::AuthError)?;

    // Verify password
    let valid =
        verify(req.password.as_bytes(), &user.password).map_err(|_| AppError::InternalError)?;

    if !valid {
        return Err(AppError::AuthError);
    }

    // Parse user level from stored string
    let level = serde_json::from_str(&user.level).unwrap_or(UserLevel::Guest);

    // Generate token
    let token = create_token(user.id.to_string(), level).map_err(|_| AppError::InternalError)?;

    Ok(Json(AuthResponse { token }))
}

/// Access a protected route (requires valid JWT token)
///
/// curl -X GET http://localhost:5000/api/protected \
///   -H "Authorization: Bearer your-secret-key"
pub async fn protected(headers: HeaderMap) -> Result<Json<&'static str>, AppError> {
    // Extract token from Authorization header
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(AppError::AuthError)?;

    // Validate JWT token
    match validate_token(auth_header) {
        Ok(token_data) => {
            let _level = token_data.claims.level;
            let _uid = token_data.claims.user_id;
            Ok(Json("Welcome '{_uid}' to protected route with '{_level}'!"))
        }
        Err(_) => return Err(AppError::AuthError),
    }
}
