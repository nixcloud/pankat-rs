mod auth;
mod db;
mod error;
mod handlers;
mod schema;
mod file_monitor;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use notify::{Watcher, RecursiveMode};
use std::path::Path;
use std::fs;
use tokio::signal;
use tokio::sync::mpsc;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create documents directory if it doesn't exist
    let documents_path = "documents";
    fs::create_dir_all(documents_path)?;

    // Setup shutdown channel
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);

    // Initialize file monitoring
    let path = Path::new(documents_path);
    let (mut watcher, mut rx) = file_monitor::async_monitor(path, shutdown_rx).await?;

    // Start watching the documents directory
    watcher.watch(path, RecursiveMode::Recursive)?;

    // Spawn file monitoring task
    let monitor_handle = tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => file_monitor::handle_event(&event),
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }
        println!("File monitor shutting down...");
    });

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
    println!("Monitoring directory: {}", documents_path);

    // Create a listener
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            if let Err(err) = signal::ctrl_c().await {
                eprintln!("Error listening for shutdown signal: {}", err);
            }
            println!("Shutdown signal received");
            let _ = shutdown_tx.send(()).await;
        })
        .await?;

    // Wait for the file monitor to complete
    monitor_handle.await?;
    println!("Graceful shutdown complete");

    Ok(())
}