mod articles;
mod auth;
mod db;
mod error;
mod file_monitor;
mod handlers;
mod registry;
mod renderer;
mod schema;

use axum::{
    routing::{get, post},
    Router,
};

use clap::{Arg, Command};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::signal;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    let default_port: u16 = 5000;

    let matches = Command::new("pankat cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joachim Schiele <js@lastlog.de")
        .about("https://github.com/nixcloud/pankat - static site generator")
        .arg(
            Arg::new("input")
                .short('d')
                .long("input")
                .value_name("PATH")
                .help("An absolute input path, i.e. where the *.md files of your blog are")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("PATH")
                .help("An absolute output path, where pankat stores the generated html files")
                .required(true),
        )
        .arg(
            Arg::new("assets")
                .short('a')
                .long("assets")
                .value_name("PATH")
                .help("An absolute assets path, where js/wasm/css/... files are stored")
                .required(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help(format!("Sets the port number, default is {}", default_port))
                .default_value(format!("{}", default_port)),
        )
        .get_matches();

    let documents_path = PathBuf::from(matches.get_one::<String>("document").unwrap());
    let blog_path = PathBuf::from(matches.get_one::<String>("blog").unwrap());
    let port_number: u32 = matches
        .get_one::<String>("port")
        .unwrap()
        .parse()
        .unwrap_or(5000);

    println!("Documents Path: {:?}", documents_path);
    println!("Blog Path: {:?}", blog_path);
    println!("Port Number: {}", port_number);

    // Create documents directory if it doesn't exist
    if !documents_path.exists() {
        fs::create_dir_all(&documents_path)?;
    }

    // Setup broadcast channel for shutdown coordination
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let shutdown_rx = shutdown_tx.subscribe();

    // Initialize file monitoring
    let monitor_handle = file_monitor::spawn_async_monitor(documents_path.clone(), shutdown_rx)
        .map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e))?;

    // Initialize SQLite database with Diesel
    let pool = db::establish_connection_pool();

    // Run migrations
    let mut conn = pool
        .get()
        .map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e))?;
    if let Err(e) = conn.run_pending_migrations(MIGRATIONS) {
        eprintln!("Migration error: {:?}", e);
    }

    // Create router
    let app = Router::new()
        .route("/", get(handlers::serve_static))
        .route("/*path", get(handlers::serve_static))
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/protected", get(handlers::protected))
        .route("/ws", get(handlers::websocket_route))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("Server running on {}", addr);
    println!(
        "Monitoring directory: {}",
        documents_path.display().to_string()
    );

    // Create a listener with retry logic
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Successfully bound to port 5000");
            listener
        }
        Err(e) => {
            eprintln!("Failed to bind to port 5000: {}", e);
            // If port is in use, try to clean up and exit
            println!("Initiating cleanup sequence...");
            if let Err(send_err) = shutdown_tx.send(()) {
                eprintln!("Error broadcasting shutdown signal: {}", send_err);
            }
            // Wait for monitor to cleanup
            if let Err(e) = monitor_handle.await {
                eprintln!("Error during monitor shutdown: {}", e);
            }
            return Err(Box::<dyn std::error::Error + Send + Sync>::from(e));
        }
    };

    // Start server with graceful shutdown
    println!("Press Ctrl+C to stop the server...");
    tokio::select! {
        result = axum::serve(listener, app) => {
            if let Err(e) = result {
                eprintln!("Server error: {}", e);
                // Initiate cleanup before returning error
                if let Err(send_err) = shutdown_tx.send(()) {
                    eprintln!("Error broadcasting shutdown signal: {}", send_err);
                }
                // Wait for monitor to cleanup
                if let Err(e) = monitor_handle.await {
                    eprintln!("Error during monitor shutdown: {}", e);
                }
                return Err(Box::<dyn std::error::Error + Send + Sync>::from(e));
            }
        }
        _ = signal::ctrl_c() => {
            println!("\nReceived Ctrl+C, initiating graceful shutdown...");
            // Broadcast shutdown signal to all tasks
            if let Err(e) = shutdown_tx.send(()) {
                eprintln!("Error broadcasting shutdown signal: {}", e);
            }
        }
    }

    // Wait for the file monitor to complete its cleanup
    println!("Waiting for file monitor to complete shutdown...");
    if let Err(e) = monitor_handle.await {
        eprintln!("Error during monitor shutdown: {}", e);
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(e));
    }
    println!("Graceful shutdown complete");

    Ok(())
}
