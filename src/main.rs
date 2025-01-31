mod articles;
mod auth;
mod config;
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
use std::net::SocketAddr;
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
                .short('i')
                .long("input")
                .value_name("PATH")
                .help("Absolute path where the media/ and posts/*.md files of your blog are located")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("PATH")
                .help("Absolute path, where pankat stores the generated html files")
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
            Arg::new("database")
                .short('d')
                .long("database")
                .value_name("PATH")
                .help("An absolute path where the database is stored")
                .required(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port number where pankat listens for incoming connections for browser connections")
                .default_value(default_port.to_string()),
        )
        .get_matches();

    let config_singleton = config::Singleton::new();
    config_singleton.initialize(
        std::path::Path::new(matches.get_one::<String>("input").unwrap()),
        std::path::Path::new(matches.get_one::<String>("output").unwrap()),
        std::path::Path::new(matches.get_one::<String>("assets").unwrap()),
        std::path::Path::new(matches.get_one::<String>("database").unwrap()),
        matches
            .get_one::<String>("port")
            .unwrap()
            .parse()
            .unwrap_or(default_port),
    );
    let config = config_singleton.instance();

    println!("-------------------------------------------------");
    println!("Input Path: {:?}", config.input);
    println!("Output Path: {:?}", config.output);
    println!("Assets Path: {:?}", config.assets);
    println!("Database Path: {:?}", config.database);
    println!("Port Number: {}", config.port);
    println!("-------------------------------------------------");

    // Setup broadcast channel for shutdown coordination
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let shutdown_rx = shutdown_tx.subscribe();

    // Initialize file monitoring
    let monitor_handle = file_monitor::spawn_async_monitor(config.input.clone(), shutdown_rx)
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
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Server running on {}", addr);
    println!("Monitoring directory: {}", config.input.display());

    // Create a listener with retry logic
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Successfully bound to port {}", config.port);
            listener
        }
        Err(e) => {
            eprintln!("Failed to bind to port {}: {}", config.port, e);
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
