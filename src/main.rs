mod auth;
mod db;
mod error;
mod handlers;
mod schema;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize SQLite database with Diesel
    let pool = db::establish_connection_pool();

    // Run migrations
    pool.get()?.run_pending_migrations(MIGRATIONS)?;

    // Create router
    let app = Router::new()
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/protected", get(handlers::protected))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("Server running on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app
    ).await?;

    Ok(())
}