//! requests against the complete router, with an in-memory database
use crate::articles::ArticleWithTags;
use crate::{auth, create_router, db};
use axum::body::Body;
use axum::http::{header, HeaderMap, Request, StatusCode};
use axum::Router;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use tower::ServiceExt;

fn article(name: &str, date: &str, draft: bool, special_page: bool) -> ArticleWithTags {
    ArticleWithTags {
        id: None,
        src_file_name: format!("posts/{}.mdwn", name),
        dst_file_name: format!("{}.html", name),
        hash: "".to_string(),
        title: Some(name.to_string()),
        modification_date: Some(
            chrono::NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M").unwrap(),
        ),
        summary: None,
        tags: None,
        series: None,
        draft: if draft { Some(true) } else { None },
        special_page: if special_page { Some(true) } else { None },
        anchorjs: None,
        tocify: None,
        live_updates: None,
        open_graph_image: None,
    }
}

fn app() -> Router {
    crate::test_config::init();
    // one connection, every connection would get its own in-memory database
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Pool::builder().max_size(1).build(manager).unwrap();
    let mut conn = pool.get().unwrap();
    db::initialize_schema(&mut conn);
    for (name, date, draft, special_page) in [
        ("draft_old", "2026-10-01 12:00", true, false),
        ("draft_new", "2026-10-10 12:00", true, false),
        ("published", "2026-10-05 12:00", false, false),
        ("about", "2026-09-01 12:00", false, true),
        ("secret_page", "2026-09-02 12:00", true, true),
    ] {
        let article = article(name, date, draft, special_page);
        db::article::set(&mut conn, &article).unwrap();
        let html = format!("<p>body of {}</p>", name);
        db::cache::set_cache(&mut conn, article.src_file_name, html, "".to_string()).unwrap();
    }
    drop(conn);
    create_router(pool, "")
}

fn admin_cookie() -> String {
    format!("{}={}", auth::SESSION_COOKIE, auth::create_session().unwrap())
}

async fn request(
    app: &Router,
    method: &str,
    uri: &str,
    cookie: Option<&str>,
    form: Option<&str>,
) -> (StatusCode, HeaderMap, String) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    let body = match form {
        Some(form) => {
            builder = builder.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
            Body::from(form.to_string())
        }
        None => Body::empty(),
    };
    let response = app.clone().oneshot(builder.body(body).unwrap()).await.unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, headers, String::from_utf8_lossy(&bytes).to_string())
}

fn location(headers: &HeaderMap) -> &str {
    headers.get(header::LOCATION).unwrap().to_str().unwrap()
}

#[tokio::test]
async fn test_drafts_need_login() {
    let app = app();
    let (status, headers, _) = request(&app, "GET", "/drafts/", None, None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location(&headers), "/login?next=%2Fdrafts%2F");

    let (status, headers, _) = request(&app, "GET", "/drafts/draft_new.html", None, None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location(&headers), "/login?next=%2Fdrafts%2Fdraft_new.html");

    let (status, headers, _) = request(&app, "GET", "/drafts", None, None).await;
    assert_eq!(status, StatusCode::PERMANENT_REDIRECT);
    assert_eq!(location(&headers), "/drafts/");
}

#[tokio::test]
async fn test_drafts_as_admin() {
    let app = app();
    let cookie = admin_cookie();

    let (status, _, body) = request(&app, "GET", "/drafts/", Some(&cookie), None).await;
    assert_eq!(status, StatusCode::OK);
    let new = body.find("/drafts/draft_new.html").expect("draft_new listed");
    let old = body.find("/drafts/draft_old.html").expect("draft_old listed");
    assert!(new < old, "newest draft first");
    assert!(!body.contains("published.html"));

    let (status, _, index) = request(&app, "GET", "/drafts/index.html", Some(&cookie), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(index, body);

    let (status, _, body) =
        request(&app, "GET", "/drafts/draft_new.html", Some(&cookie), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<p>body of draft_new</p>"));
    assert!(!body.contains("og:"), "no social preview for drafts");

    // published articles aren't served as drafts
    let (status, _, body) =
        request(&app, "GET", "/drafts/published.html", Some(&cookie), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(!body.contains("body of published"));

    // a forged or foreign cookie is no session
    let (status, _, _) = request(
        &app,
        "GET",
        "/drafts/",
        Some("pankat_session=eyJhbGciOiJub25lIn0.eyJzdWIiOiJhZG1pbiJ9."),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::SEE_OTHER);
}

#[tokio::test]
async fn test_login() {
    let app = app();
    let (_, _, body) = request(&app, "GET", "/api/session", None, None).await;
    assert_eq!(body, r#"{"admin":false}"#);

    let (status, _, body) = request(
        &app,
        "POST",
        "/login",
        None,
        Some("password=wrong&next=%2Fdrafts%2F"),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(body.contains("wrong password"));

    let form = format!(
        "password={}&next=%2Fdrafts%2Fdraft_new.html",
        crate::test_config::ADMIN_PASSWORD
    );
    let (status, headers, _) = request(&app, "POST", "/login", None, Some(&form)).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location(&headers), "/drafts/draft_new.html");
    let set_cookie = headers.get(header::SET_COOKIE).unwrap().to_str().unwrap();
    assert!(set_cookie.contains("HttpOnly"));
    assert!(set_cookie.contains("Max-Age=4320000"));
    let cookie = set_cookie.split(';').next().unwrap();

    let (_, _, body) = request(&app, "GET", "/api/session", Some(cookie), None).await;
    assert_eq!(body, r#"{"admin":true}"#);

    let (status, headers, _) = request(&app, "POST", "/logout", Some(cookie), None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(headers
        .get(header::SET_COOKIE)
        .unwrap()
        .to_str()
        .unwrap()
        .contains("Max-Age=0"));
}

#[tokio::test]
async fn test_not_found() {
    let app = app();
    for uri in ["/does-not-exist.html", "/../../pankat.toml", "/posts/../../pankat.toml"] {
        let (status, _, body) = request(&app, "GET", uri, None, None).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{}", uri);
        assert!(body.contains("page not found"));
        assert!(body.contains(r#"<a href="/index.html">"#));
        assert!(!body.contains("admin_password"));
    }
}

#[tokio::test]
async fn test_special_pages() {
    let app = app();
    let (status, headers, _) = request(&app, "GET", "/specialpages/", None, None).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(location(&headers), "/login?next=%2Fspecialpages%2F");

    let (status, headers, _) = request(&app, "GET", "/specialpages", None, None).await;
    assert_eq!(status, StatusCode::PERMANENT_REDIRECT);
    assert_eq!(location(&headers), "/specialpages/");

    let cookie = admin_cookie();
    let (status, _, body) = request(&app, "GET", "/specialpages/", Some(&cookie), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(r#"<a href="/about.html">about</a>"#));
    // a special page which is a draft as well is only reachable below drafts/
    assert!(body.contains(r#"<a href="/drafts/secret_page.html">secret_page</a>"#));
    assert!(!body.contains("published.html"));
    assert!(!body.contains("draft_new.html"));

    let (status, _, index) =
        request(&app, "GET", "/specialpages/index.html", Some(&cookie), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(index, body);
}

async fn webhook(app: &Router, event: &str, signature: Option<&str>) -> StatusCode {
    let body = r#"{"zen":"Keep it logically awesome.","hook_id":1}"#;
    let mut builder = Request::builder()
        .method("POST")
        .uri("/github-webhook")
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-GitHub-Event", event);
    let signature = signature
        .map(str::to_string)
        .unwrap_or_else(|| crate::github_webhook::sign(crate::test_config::GITHUB_WEBHOOK_SECRET, body.as_bytes()));
    if !signature.is_empty() {
        builder = builder.header("X-Hub-Signature-256", signature);
    }
    let response = app.clone().oneshot(builder.body(Body::from(body)).unwrap()).await.unwrap();
    response.status()
}

// a push runs git pull in the input directory, documents/blog.lastlog.de, it isn't sent here
// (git_pull is tested in github_webhook.rs)
#[tokio::test]
async fn test_github_webhook() {
    let app = app();
    assert_eq!(webhook(&app, "ping", None).await, StatusCode::OK);
    assert_eq!(webhook(&app, "issues", None).await, StatusCode::NO_CONTENT);
    assert_eq!(webhook(&app, "push", Some("")).await, StatusCode::UNAUTHORIZED);
    let other_secret = crate::github_webhook::sign("another secret", b"{}");
    assert_eq!(webhook(&app, "push", Some(&other_secret)).await, StatusCode::UNAUTHORIZED);
    // only POST
    let (status, _, _) = request(&app, "GET", "/github-webhook", None, None).await;
    assert_ne!(status, StatusCode::OK);
}
