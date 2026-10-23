use crate::auth::{self, Admin};
use crate::articles;
use crate::config;
use crate::db::DbPool;
use crate::error::AppError;
use crate::registry::*;
use axum::extract::ws::{Message, WebSocket};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{
    extract::{ws::WebSocketUpgrade, Query, State},
    Form, Json,
};
use serde::Deserialize;
use serde_json::Value;
use std::path::{Component, Path, PathBuf};
use tokio::fs;
use tokio::time;
use tokio::time::Duration;

#[derive(Deserialize)]
pub struct LoginQuery {
    next: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    password: String,
    next: Option<String>,
}

fn login_page(next: &str, error: Option<&str>) -> Result<String, AppError> {
    let cfg = config::Config::get();
    let content = if auth::login_enabled() {
        format!(
            r#"<h1 id="SiteTitle">admin login</h1>
               <p>this blog has a single user, the administrator. sign in with the
               <code>admin_password</code> from pankat.toml.</p>
               {}
               <form method="post" action="{}/login" class="form-inline">
                 <input type="hidden" name="next" value="{}" />
                 <input type="password" name="password" class="form-control"
                   placeholder="password" autofocus required />
                 <button type="submit" class="btn btn-primary">sign in</button>
               </form>"#,
            error.map_or(String::new(), |e| format!(
                r#"<p class="alert alert-danger">{}</p>"#,
                e
            )),
            cfg.subdir.display(),
            handlebars::html_escape(next),
        )
    } else {
        r#"<h1 id="SiteTitle">admin login</h1>
           <p>the login is disabled, set <code>admin_password</code> and
           <code>jwt_token</code> in pankat.toml to enable it.</p>"#
            .to_string()
    };
    crate::renderer::html::create_html_page("admin login", &content)
        .map_err(|_| AppError::InternalError)
}

fn html_response(status: StatusCode, html: String) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        html,
    )
        .into_response()
}

pub async fn login_form(
    admin: Option<Admin>,
    Query(query): Query<LoginQuery>,
) -> Result<Response, AppError> {
    let next = auth::sanitize_next(query.next.as_deref());
    if admin.is_some() {
        return Ok(Redirect::to(&next).into_response());
    }
    Ok(html_response(StatusCode::OK, login_page(&next, None)?))
}

pub async fn login(Form(form): Form<LoginForm>) -> Result<Response, AppError> {
    let next = auth::sanitize_next(form.next.as_deref());
    if !auth::check_password(&form.password) {
        // slows down guessing
        time::sleep(Duration::from_secs(1)).await;
        let html = login_page(&next, Some("wrong password"))?;
        return Ok(html_response(StatusCode::UNAUTHORIZED, html));
    }
    let token = auth::create_session().map_err(|_| AppError::InternalError)?;
    Ok((
        [(header::SET_COOKIE, auth::session_cookie(&token))],
        Redirect::to(&next),
    )
        .into_response())
}

pub async fn logout() -> Response {
    let cfg = config::Config::get();
    (
        [(header::SET_COOKIE, auth::logout_cookie())],
        Redirect::to(&format!("{}/index.html", cfg.subdir.display())),
    )
        .into_response()
}

/// for the login/logout buttons, the pages themselves are static
pub async fn session(admin: Option<Admin>) -> Json<Value> {
    Json(serde_json::json!({ "admin": admin.is_some() }))
}

pub async fn drafts_redirect() -> Redirect {
    let cfg = config::Config::get();
    Redirect::permanent(&format!("{}/drafts/", cfg.subdir.display()))
}

pub async fn drafts_index(_: Admin, State(pool): State<DbPool>) -> Result<Response, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let html = articles::create_drafts_index(&mut conn).map_err(|_| AppError::InternalError)?;
    Ok(html_response(StatusCode::OK, html))
}

/// drafts are rendered on request, never written to the (public) output; relative links in a
/// draft (posts/.., media/.., assets) resolve below drafts/ and are served from where they live
pub async fn draft(
    admin: Admin,
    State(pool): State<DbPool>,
    uri: axum::http::Uri,
) -> Result<Response, AppError> {
    let path = uri.path().strip_prefix("/drafts/").unwrap_or_default();
    if path == "index.html" {
        return drafts_index(admin, State(pool)).await;
    }
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    if let Some(html) = articles::create_draft_page(&mut conn, path) {
        return Ok(html_response(StatusCode::OK, html));
    }
    let cfg = config::Config::get();
    let base = if path.starts_with("posts/") || path.starts_with("media/") {
        &cfg.input
    } else {
        &cfg.output
    };
    serve_file(safe_join(base, path)).await
}

pub async fn special_pages_redirect() -> Redirect {
    let cfg = config::Config::get();
    Redirect::permanent(&format!("{}/specialpages/", cfg.subdir.display()))
}

pub async fn special_pages_index(
    _: Admin,
    State(pool): State<DbPool>,
) -> Result<Response, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    let html =
        articles::create_special_pages_index(&mut conn).map_err(|_| AppError::InternalError)?;
    Ok(html_response(StatusCode::OK, html))
}

/// `uri_path` below `base`, None if it would leave `base` (e.g. /../../pankat.toml)
fn safe_join(base: &Path, uri_path: &str) -> Option<PathBuf> {
    let mut path = base.to_path_buf();
    for component in Path::new(uri_path.trim_start_matches('/')).components() {
        match component {
            Component::Normal(c) => path.push(c),
            Component::CurDir => {}
            _ => return None,
        }
    }
    Some(path)
}

async fn serve_file(path: Option<PathBuf>) -> Result<Response, AppError> {
    let path = path.ok_or(AppError::NotFound)?;
    let contents = fs::read(&path).await.map_err(|_| AppError::NotFound)?;
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type.as_ref())
        .body(contents.into())
        .map_err(|_| AppError::InternalError)
}

pub async fn serve_index() -> Result<Response, AppError> {
    let cfg = config::Config::get();
    serve_file(Some(cfg.output.join("index.html"))).await
}

pub async fn serve_output(uri: axum::http::Uri) -> Result<Response, AppError> {
    println!("Received request for URI (serve_output): {}", uri);
    let cfg = config::Config::get();
    serve_file(safe_join(&cfg.output, uri.path())).await
}

pub async fn serve_input(uri: axum::http::Uri) -> Result<Response, AppError> {
    println!("Received request for URI (serve_input): {}", uri);
    let cfg = config::Config::get();
    serve_file(safe_join(&cfg.input, uri.path())).await
}

pub async fn serve_internals(uri: axum::http::Uri) -> Result<Response, AppError> {
    println!("Received request for URI (serve_internals): {}", uri);
    let cfg = config::Config::get();
    let path = if let Some(p) = uri.path().strip_prefix("/assets/") {
        safe_join(&cfg.assets, p)
    } else if let Some(p) = uri.path().strip_prefix("/wasm/") {
        safe_join(&cfg.wasm, p)
    } else {
        None
    };
    serve_file(path).await
}

pub async fn not_found() -> AppError {
    AppError::NotFound
}

pub async fn websocket_route(admin: Option<Admin>, ws: WebSocketUpgrade) -> Response {
    println!("Received request for new ws connection request");
    // the browser sends the session cookie along with the upgrade request
    let admin: bool = admin.is_some();
    ws.on_upgrade(move |socket| handle_socket(socket, admin))
}

/// unwraps {"admin": message} / {"public": message} for the matching audience, drops it for
/// the other one and passes everything else on; this keeps drafts away from the public
fn message_for_audience(message: String, admin: bool) -> Option<String> {
    let (own, other) = if admin {
        ("admin", "public")
    } else {
        ("public", "admin")
    };
    match serde_json::from_str::<Value>(&message) {
        Ok(Value::Object(object)) if object.len() == 1 => {
            if let Some(inner) = object.get(own) {
                Some(inner.to_string())
            } else if object.contains_key(other) {
                None
            } else {
                Some(message)
            }
        }
        _ => Some(message),
    }
}

async fn handle_socket(mut socket: WebSocket, admin: bool) {
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
                        let Some(message) = message_for_audience(message, admin) else {
                            continue;
                        };
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

#[cfg(test)]
mod tests {
    use super::safe_join;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_safe_join() {
        let base = Path::new("documents/output");
        assert_eq!(
            safe_join(base, "/index.html"),
            Some(PathBuf::from("documents/output/index.html"))
        );
        assert_eq!(
            safe_join(base, "/posts/./libnix/x.png"),
            Some(PathBuf::from("documents/output/posts/libnix/x.png"))
        );
        assert_eq!(safe_join(base, "/../../pankat.toml"), None);
        assert_eq!(safe_join(base, "/posts/../../pankat.toml"), None);
        assert_eq!(safe_join(base, "/posts/.."), None);
    }
}

#[cfg(test)]
mod audience_tests {
    use super::message_for_audience;

    #[test]
    fn test_message_for_audience() {
        let admin = r#"{"admin":{"update":"draft"}}"#.to_string();
        let public = r#"{"public":{"update":"removed"}}"#.to_string();
        let all = r#"{"update":"article"}"#.to_string();
        assert_eq!(
            message_for_audience(admin.clone(), true).as_deref(),
            Some(r#"{"update":"draft"}"#)
        );
        assert_eq!(message_for_audience(admin, false), None);
        assert_eq!(message_for_audience(public.clone(), true), None);
        assert_eq!(
            message_for_audience(public, false).as_deref(),
            Some(r#"{"update":"removed"}"#)
        );
        assert_eq!(message_for_audience(all.clone(), true), Some(all.clone()));
        assert_eq!(message_for_audience(all.clone(), false), Some(all));
    }
}
