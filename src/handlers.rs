use crate::auth::{create_token, validate_token, UserLevel};
use crate::config;
use crate::db::users::{create_user, get_user_by_username};
use crate::error::AppError;
use crate::registry::*;
use axum::extract::ws::{Message, WebSocket};
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::header::{HeaderMap, AUTHORIZATION},
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use tokio::fs;
use tokio::time;
use tokio::time::Duration;

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

pub async fn serve_output(uri: axum::http::Uri) -> Result<Response, AppError> {
    println!("Received request for URI (serve_output): {}", uri);
    let cfg = config::Config::get();
    let mut path = PathBuf::from(cfg.output.clone());

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

            let file_path = PathBuf::from(cfg.output.clone()).join("index.html");
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

pub async fn serve_input(uri: axum::http::Uri) -> Result<Response, AppError> {
    println!("Received request for URI (serve_input): {}", uri);
    let cfg = config::Config::get();
    let mut input = PathBuf::from(cfg.input.clone());

    let uri_path = PathBuf::from(uri.path());
    let path_str = uri_path
        .strip_prefix("/")
        .map_err(|_| AppError::InternalError)?;

    input.push(&path_str);

    match fs::read(&input).await {
        Ok(contents) => {
            let mime_type = mime_guess::from_path(&input).first_or_text_plain();

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(contents.into())
                .map_err(|_| AppError::InternalError)?;

            Ok(response)
        }
        Err(_) => Err(AppError::InternalError),
    }
}

pub async fn serve_internals(uri: axum::http::Uri) -> Result<Response, AppError> {
    println!("Received request for URI (serve_internals): {}", uri);
    let mut path_to_serve = PathBuf::new();
    let uri_path = PathBuf::from(uri.path());
    let cfg = config::Config::get();

    if let Some(first_path) = uri.path().split('/').nth(1) {
        //println!("First path segment: {}", first_path);
        match first_path {
            "assets" => {
                path_to_serve = PathBuf::from(cfg.assets.clone());
                let path_str = uri_path
                    .strip_prefix("/assets/")
                    .map_err(|_| AppError::InternalError)?;
                path_to_serve.push(&path_str);
            }
            "wasm" => {
                path_to_serve = PathBuf::from(cfg.wasm.clone());
                let path_str = uri_path
                    .strip_prefix("/wasm/")
                    .map_err(|_| AppError::InternalError)?;
                path_to_serve.push(&path_str);
            }
            _ => {}
        }
    } else {
        return Err(AppError::InternalError);
    }

    match fs::read(&path_to_serve).await {
        Ok(contents) => {
            let mime_type = mime_guess::from_path(&path_to_serve).first_or_text_plain();

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(contents.into())
                .map_err(|_| AppError::InternalError)?;

            Ok(response)
        }
        Err(_) => Err(AppError::InternalError),
    }
}

pub async fn websocket_route(ws: WebSocketUpgrade) -> Response {
    println!("Received request for new ws connection request");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // Step 1: Wait for the initial message to determine the subscription
    let register_name = if let Some(Ok(Message::Text(text))) = socket.recv().await {
        match serde_json::from_str::<Value>(&text) {
            Ok(json) => json
                .get("register")
                .and_then(|v| v.as_str())
                .map(String::from),
            Err(_) => None,
        }
    } else {
        None
    };

    let register_name = match register_name {
        Some(name) => name,
        None => {
            println!("Invalid or missing register name, closing connection.");
            return;
        }
    };

    println!("Registering for: {}", register_name);

    // Step 2: Get the sender-receiver based on the register name
    let (_, mut receiver) = PubSubRegistry::instance()
        .get_sender_receiver_by_name(register_name)
        .await;

    let mut interval = time::interval(Duration::from_millis(5000));

    // Step 3: Enter the loop after proper initialization
    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = socket.send(Message::Text(r#"{"ping" : ""}"#.to_string())).await {
                    println!("Error sending message: {:?}", e);
                    break;
                }
            }
            msg = socket.recv() => {
                if let Some(res) = msg {
                    match res {
                        Ok(_) => continue,
                        Err(_) => {
                            //println!("WS close");
                            break;
                        },
                    }
                } else {
                    //println!("WS close");
                    break;
                }
            }
            msg = receiver.recv() => {
                match msg {
                    Ok(message) => {
                        if let Err(e) = socket.send(Message::Text(message)).await {
                            println!("Error sending data: {:?}", e);
                        }
                    }
                    Err(e) => {
                        println!("Error receiving data from article update: {:?}", e);
                    },
                }
            }
        }
    }
    //println!("WS close, loop done");
}
