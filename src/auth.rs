//! pankat has a single user, the admin: logging in compares the password with `admin_password`
//! and stores a session token, a JWT signed with `jwt_token`, in an http-only cookie
use crate::config::{self, NOT_SET};
use axum::extract::{FromRequestParts, OriginalUri};
use axum::http::{header, request::Parts, HeaderMap};
use axum::response::{IntoResponse, Redirect, Response};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const SESSION_COOKIE: &str = "pankat_session";
pub const SESSION_TTL_SECS: u64 = 50 * 24 * 60 * 60;
const ADMIN: &str = "admin";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

/// login is disabled until both secrets are set in pankat.toml (or the environment)
pub fn login_enabled() -> bool {
    let cfg = config::Config::get();
    [&cfg.admin_password, &cfg.jwt_token]
        .iter()
        .all(|s| !s.is_empty() && s.as_str() != NOT_SET)
}

/// compares in constant time, so the response time doesn't tell how much of it was right
pub fn check_password(password: &str) -> bool {
    let cfg = config::Config::get();
    let expected = cfg.admin_password.as_bytes();
    let given = password.as_bytes();
    let difference = given
        .iter()
        .zip(expected.iter())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b));
    login_enabled() && given.len() == expected.len() && difference == 0
}

pub fn create_session() -> Result<String, jsonwebtoken::errors::Error> {
    let cfg = config::Config::get();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let claims = Claims {
        sub: ADMIN.to_string(),
        exp: now + SESSION_TTL_SECS,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_token.as_bytes()),
    )
}

pub fn validate_session(token: &str) -> bool {
    let cfg = config::Config::get();
    login_enabled()
        && decode::<Claims>(
            token,
            &DecodingKey::from_secret(cfg.jwt_token.as_bytes()),
            &Validation::default(),
        )
        .map_or(false, |data| data.claims.sub == ADMIN)
}

fn session_from_headers(headers: &HeaderMap) -> Option<&str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|cookie| cookie.trim().split_once('='))
        .find(|(name, _)| *name == SESSION_COOKIE)
        .map(|(_, value)| value)
}

fn cookie(value: &str, max_age: u64) -> String {
    let cfg = config::Config::get();
    let secure = if cfg.site_url.starts_with("https://") {
        "; Secure"
    } else {
        ""
    };
    format!(
        "{}={}; HttpOnly; SameSite=Lax; Path={}/; Max-Age={}{}",
        SESSION_COOKIE,
        value,
        cfg.subdir.display(),
        max_age,
        secure
    )
}

pub fn session_cookie(token: &str) -> String {
    cookie(token, SESSION_TTL_SECS)
}

pub fn logout_cookie() -> String {
    cookie("", 0)
}

/// only local paths below subdir are accepted as target after the login, everything
/// else (e.g. //evil.example) falls back to the drafts
pub fn sanitize_next(next: Option<&str>) -> String {
    let cfg = config::Config::get();
    let prefix = format!("{}/", cfg.subdir.display());
    match next {
        Some(next)
            if next.starts_with(&prefix)
                && !next.starts_with("//")
                && !next.contains('\\')
                && !next.chars().any(char::is_control) =>
        {
            next.to_string()
        }
        _ => format!("{}drafts/", prefix),
    }
}

/// a handler taking `Admin` is only reachable with a valid session, everybody else is
/// redirected to the login, which afterwards returns to the requested page;
/// `Option<Admin>` makes it optional
pub struct Admin;

#[axum::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Admin {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if session_from_headers(&parts.headers).map_or(false, validate_session) {
            return Ok(Admin);
        }
        // inside the nested router, uri lacks the subdir
        let uri = parts
            .extensions
            .get::<OriginalUri>()
            .map_or(&parts.uri, |original| &original.0);
        let next = uri.path_and_query().map_or(uri.path(), |p| p.as_str());
        let cfg = config::Config::get();
        let query = serde_urlencoded::to_string([("next", next)]).unwrap_or_default();
        Err(Redirect::to(&format!("{}/login?{}", cfg.subdir.display(), query)).into_response())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        crate::test_config::init();
        let token = create_session().unwrap();
        assert!(validate_session(&token));
        assert!(!validate_session("garbage"));

        // signed with another secret
        let foreign = encode(
            &Header::default(),
            &Claims {
                sub: ADMIN.to_string(),
                exp: u64::MAX / 2,
            },
            &EncodingKey::from_secret(b"another secret"),
        )
        .unwrap();
        assert!(!validate_session(&foreign));

        // expired
        let expired = encode(
            &Header::default(),
            &Claims {
                sub: ADMIN.to_string(),
                exp: 1,
            },
            &EncodingKey::from_secret(config::Config::get().jwt_token.as_bytes()),
        )
        .unwrap();
        assert!(!validate_session(&expired));
    }

    #[test]
    fn test_check_password() {
        crate::test_config::init();
        assert!(check_password(crate::test_config::ADMIN_PASSWORD));
        assert!(!check_password("wrong"));
        assert!(!check_password(""));
        assert!(!check_password(&format!("{}x", crate::test_config::ADMIN_PASSWORD)));
    }

    #[test]
    fn test_sanitize_next() {
        crate::test_config::init();
        assert_eq!(sanitize_next(Some("/drafts/foo.html")), "/drafts/foo.html");
        assert_eq!(sanitize_next(None), "/drafts/");
        assert_eq!(sanitize_next(Some("//evil.example/")), "/drafts/");
        assert_eq!(sanitize_next(Some("https://evil.example/")), "/drafts/");
        assert_eq!(sanitize_next(Some("/\\evil.example")), "/drafts/");
    }

    #[test]
    fn test_session_cookie_parsing() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            "a=b; pankat_session=tok.en.x; c=d".parse().unwrap(),
        );
        assert_eq!(session_from_headers(&headers), Some("tok.en.x"));
        assert_eq!(session_from_headers(&HeaderMap::new()), None);
    }
}
