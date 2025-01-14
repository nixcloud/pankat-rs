use crate::auth::{create_token, validate_token, UserLevel};
use crate::db::{create_user, get_user_by_username};
use crate::error::AppError;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    http::header::{HeaderMap, AUTHORIZATION},
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Deserialize)]
pub struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

pub async fn register(
    State(pool): State<DbPool>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

    let hashed =
        hash(req.password.as_bytes(), DEFAULT_COST).map_err(|_| AppError::InternalError)?;

    let user_id = create_user(&mut conn, &req.username, &hashed, UserLevel::User)
        .map_err(AppError::DatabaseError)?;

    let token =
        create_token(user_id.to_string(), UserLevel::User).map_err(|_| AppError::InternalError)?;

    Ok(Json(AuthResponse { token }))
}

pub async fn login(
    State(pool): State<DbPool>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;

    let user = get_user_by_username(&mut conn, &req.username)
        .map_err(AppError::DatabaseError)?
        .ok_or(AppError::AuthError)?;

    let valid =
        verify(req.password.as_bytes(), &user.password).map_err(|_| AppError::InternalError)?;

    if !valid {
        return Err(AppError::AuthError);
    }

    let level = serde_json::from_str(&user.level).unwrap_or(UserLevel::Guest);

    let token = create_token(user.id.to_string(), level).map_err(|_| AppError::InternalError)?;

    Ok(Json(AuthResponse { token }))
}

pub async fn protected(headers: HeaderMap) -> Result<Json<&'static str>, AppError> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(AppError::AuthError)?;

    match validate_token(auth_header) {
        Ok(token_data) => {
            let _level = token_data.claims.level;
            let _uid = token_data.claims.user_id;
            Ok(Json("Welcome '{_uid}' to protected route with '{_level}'!"))
        }
        Err(_) => return Err(AppError::AuthError),
    }
}

use axum::http::{header, StatusCode};
use axum::response::Response;
use std::path::PathBuf;
use tokio::fs;

/// Serve index.html or static files
pub async fn serve_static(uri: axum::http::Uri) -> Result<Response, AppError> {
    let mut path = PathBuf::from("documents");

    let path_str = uri.path();
    if path_str == "/" {
        path.push("index.html");
    } else {
        path.push(&path_str[1..]);
    }

    match fs::read(&path).await {
        Ok(contents) => {
            let mime_type = mime_guess::from_path(&path).first_or_text_plain();

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(contents.into())
                .map_err(|_| AppError::InternalError)?;

            Ok(response)
        }
        Err(_) => {
            if path_str != "/" {
                return Err(AppError::InternalError);
            }

            let file_path = PathBuf::from("documents/index.html");
            let html = fs::read_to_string(file_path)
                .await
                .map_err(|_| AppError::InternalError)?;

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html")
                .body(html.into())
                .map_err(|_| AppError::InternalError)?;

            Ok(response)
        }
    }
}

pub async fn websocket_route(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    use rand::Rng;
    use std::time::Duration;
    use tokio::time::interval;

    let mut interval = interval(Duration::from_secs(5));

    while let Some(Ok(msg)) = socket.recv().await {
        let response = if rand::thread_rng().gen_bool(0.5) {
            "<p>hi</p>"
        } else {
            "<p>yes</p>"
        };

        if socket.send(axum::extract::ws::Message::Text(response.to_string())).await.is_err() {
            return;
        }
    }
}